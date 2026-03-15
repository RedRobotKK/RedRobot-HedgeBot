//! PostgreSQL database layer — async connection pool, migrations, typed queries.
//!
//! ## Architecture
//!
//! All database I/O goes through the [`Database`] struct, which wraps a
//! [`sqlx::PgPool`] (connection pool, default 12 connections).  The pool is
//! shared via `Arc<Database>` across the Axum web handlers and trading loop.
//!
//! ## Startup sequence
//!
//! 1. `Database::connect()` opens the pool and verifies connectivity.
//! 2. Embedded migrations from `./migrations/` are applied idempotently.
//! 3. `Arc<Database>` is injected into both `AppState` and `run_cycle`.
//!
//! ## Graceful degradation
//!
//! If `DATABASE_URL` is missing or the server is unreachable, the bot logs a
//! warning and continues without persistence.  Every call site checks
//! `Option<Arc<Database>>` so the trading loop never blocks on a DB outage.
//!
//! ## AI / MCP access
//!
//! `closed_trades.signal_contrib` and `positions.signal_contrib` are `JSONB`.
//! With the PostgreSQL MCP server wired into Claude, the admin can ask:
//!
//! ```text
//! "What's the avg R-multiple for trades where RSI was bullish vs bearish?"
//! ```
//! Claude translates this to SQL and executes it directly against the live DB.
//!
//! ## Ollama integration
//!
//! When `OLLAMA_BASE_URL` is set (default: `http://localhost:11434`), the
//! `query_ollama()` function sends prompts to a local LLM (e.g. `llama3.2`)
//! for on-device analysis without Anthropic API costs.  Future: daily trade
//! summaries → Ollama → stored in `ai_analyses` table.

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::sync::Arc;

// ─────────────────────────────────────────────────────────────────────────────
//  Public result types
// ─────────────────────────────────────────────────────────────────────────────

/// One point in the TVL graph — returned by `get_aum_history()`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AumPoint {
    pub recorded_at:       DateTime<Utc>,
    pub total_aum:         f64,
    pub deposited_capital: f64,
    pub total_pnl:         f64,
    pub pnl_pct:           f64,
    pub active_tenants:    i32,
    pub open_positions:    i32,
}

/// Data written at the end of each trading cycle by `insert_aum_snapshot()`.
#[derive(Debug, Clone)]
pub struct AumSnapshot {
    pub total_aum:            f64,
    pub deposited_capital:    f64,
    pub total_pnl:            f64,
    pub pnl_pct:              f64,
    pub active_tenant_count:  i32,
    pub total_tenant_count:   i32,
    pub open_position_count:  i32,
    pub total_trades_today:   i32,
    pub win_rate_today:       Option<f64>,
}

/// Most-recent AUM row — for the admin headline numbers.
#[derive(Debug, Clone, Serialize)]
pub struct AumSummary {
    pub total_aum:         f64,
    pub deposited_capital: f64,
    pub total_pnl:         f64,
    pub pnl_pct:           f64,
    pub active_tenants:    i32,
    pub total_tenants:     i32,
    pub open_positions:    i32,
    pub recorded_at:       DateTime<Utc>,
}

// ─────────────────────────────────────────────────────────────────────────────
//  Database
// ─────────────────────────────────────────────────────────────────────────────

/// Async PostgreSQL connection pool. Clone is cheap — all clones share the pool.
#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

/// `Arc`-wrapped `Database` — the canonical way to share it across tasks.
pub type SharedDb = Arc<Database>;

impl Database {
    // ── Lifecycle ──────────────────────────────────────────────────────────────

    /// Open the connection pool and run pending migrations.
    pub async fn connect(url: &str) -> Result<Self> {
        log::info!("🗄  Connecting to PostgreSQL…");
        let pool = PgPoolOptions::new()
            .max_connections(12)  // 10 Axum handlers + 2 trading-loop writers
            .min_connections(2)   // keep warm — avoids first-query latency
            .acquire_timeout(std::time::Duration::from_secs(8))
            .connect(url)
            .await
            .with_context(|| format!("Cannot connect to PostgreSQL: {url}"))?;

        let db = Database { pool };
        db.run_migrations().await?;
        log::info!("✅ PostgreSQL ready");
        Ok(db)
    }

    /// Apply all pending migrations from `./migrations/` (embedded at compile time).
    /// Idempotent — tracks applied files in `_sqlx_migrations`.
    async fn run_migrations(&self) -> Result<()> {
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await
            .context("Database migration failed")?;
        log::info!("🗄  Migrations verified");
        Ok(())
    }

    /// Health-check ping.
    pub async fn ping(&self) -> Result<()> {
        sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await
            .context("DB ping failed")?;
        Ok(())
    }

    /// Expose the raw pool for advanced callers (tests, raw queries).
    pub fn pool(&self) -> &PgPool { &self.pool }

    // ── Equity snapshots ────────────────────────────────────────────────────────

    /// Record one equity snapshot for a tenant.
    /// Called every 30 s per tenant from the trading loop.
    pub async fn insert_equity_snapshot(
        &self,
        tenant_id: &str,
        equity:    f64,
    ) -> Result<()> {
        sqlx::query(
            "INSERT INTO equity_snapshots (tenant_id, equity) VALUES ($1::uuid, $2)"
        )
        .bind(tenant_id)
        .bind(equity)
        .execute(&self.pool)
        .await
        .with_context(|| format!("insert_equity_snapshot: tenant={tenant_id}"))?;
        Ok(())
    }

    /// Fetch the most recent `limit` equity snapshots for a tenant.
    /// Returns chronological order (oldest first) — ready for SVG point builder.
    pub async fn get_equity_history(
        &self,
        tenant_id: &str,
        limit:     i64,
    ) -> Result<Vec<(DateTime<Utc>, f64)>> {
        struct Row { recorded_at: DateTime<Utc>, equity: Option<f64> }

        let rows = sqlx::query_as!(
            Row,
            r#"
            SELECT recorded_at, equity::float8 AS equity
            FROM   equity_snapshots
            WHERE  tenant_id = $1::uuid
            ORDER  BY recorded_at DESC
            LIMIT  $2
            "#,
            tenant_id,
            limit,
        )
        .fetch_all(&self.pool)
        .await
        .with_context(|| format!("get_equity_history: tenant={tenant_id}"))?;

        let mut pts: Vec<(DateTime<Utc>, f64)> = rows
            .into_iter()
            .filter_map(|r| Some((r.recorded_at, r.equity?)))
            .collect();
        pts.reverse();  // ascending time order
        Ok(pts)
    }

    // ── AUM snapshots ───────────────────────────────────────────────────────────

    /// Write the pre-aggregated AUM snapshot at the end of a trading cycle.
    /// This single write makes every admin/TVL query O(1).
    pub async fn insert_aum_snapshot(&self, s: &AumSnapshot) -> Result<()> {
        sqlx::query(
            r#"INSERT INTO aum_snapshots (
                total_aum, deposited_capital, total_pnl, pnl_pct,
                active_tenant_count, total_tenant_count,
                open_position_count, total_trades_today, win_rate_today
            ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)"#
        )
        .bind(s.total_aum)
        .bind(s.deposited_capital)
        .bind(s.total_pnl)
        .bind(s.pnl_pct)
        .bind(s.active_tenant_count)
        .bind(s.total_tenant_count)
        .bind(s.open_position_count)
        .bind(s.total_trades_today)
        .bind(s.win_rate_today)
        .execute(&self.pool)
        .await
        .context("insert_aum_snapshot failed")?;
        Ok(())
    }

    /// Return AUM time-series for the TVL graph.
    /// `days=90` for landing page; `days=3650` for all-time admin view.
    pub async fn get_aum_history(&self, days: i32) -> Result<Vec<AumPoint>> {
        // Build interval string outside the query to avoid type inference issues.
        let interval = format!("{days} days");

        struct Row {
            recorded_at:       DateTime<Utc>,
            total_aum:         Option<f64>,
            deposited_capital: Option<f64>,
            total_pnl:         Option<f64>,
            pnl_pct:           Option<f64>,
            active_tenants:    i32,
            open_positions:    i32,
        }

        let rows = sqlx::query_as!(
            Row,
            r#"
            SELECT
                recorded_at,
                total_aum::float8         AS total_aum,
                deposited_capital::float8 AS deposited_capital,
                total_pnl::float8         AS total_pnl,
                pnl_pct::float8           AS pnl_pct,
                active_tenant_count       AS active_tenants,
                open_position_count       AS open_positions
            FROM  aum_snapshots
            WHERE recorded_at > now() - $1::interval
            ORDER BY recorded_at ASC
            "#,
            interval,
        )
        .fetch_all(&self.pool)
        .await
        .context("get_aum_history failed")?;

        Ok(rows.into_iter().filter_map(|r| {
            Some(AumPoint {
                recorded_at:       r.recorded_at,
                total_aum:         r.total_aum?,
                deposited_capital: r.deposited_capital?,
                total_pnl:         r.total_pnl?,
                pnl_pct:           r.pnl_pct?,
                active_tenants:    r.active_tenants,
                open_positions:    r.open_positions,
            })
        }).collect())
    }

    /// Most recent AUM row for headline display.
    pub async fn get_latest_aum(&self) -> Result<Option<AumSummary>> {
        struct Row {
            recorded_at:         DateTime<Utc>,
            total_aum:           Option<f64>,
            deposited_capital:   Option<f64>,
            total_pnl:           Option<f64>,
            pnl_pct:             Option<f64>,
            active_tenant_count: i32,
            total_tenant_count:  i32,
            open_position_count: i32,
        }

        let row = sqlx::query_as!(
            Row,
            r#"
            SELECT recorded_at,
                   total_aum::float8         AS total_aum,
                   deposited_capital::float8 AS deposited_capital,
                   total_pnl::float8         AS total_pnl,
                   pnl_pct::float8           AS pnl_pct,
                   active_tenant_count,
                   total_tenant_count,
                   open_position_count
            FROM   aum_snapshots
            ORDER  BY recorded_at DESC
            LIMIT  1
            "#
        )
        .fetch_optional(&self.pool)
        .await
        .context("get_latest_aum failed")?;

        Ok(row.and_then(|r| Some(AumSummary {
            total_aum:         r.total_aum?,
            deposited_capital: r.deposited_capital?,
            total_pnl:         r.total_pnl?,
            pnl_pct:           r.pnl_pct?,
            active_tenants:    r.active_tenant_count,
            total_tenants:     r.total_tenant_count,
            open_positions:    r.open_position_count,
            recorded_at:       r.recorded_at,
        })))
    }

    // ── Closed trades ───────────────────────────────────────────────────────────

    /// Append an immutable closed-trade record.
    /// `signal_contrib` is stored as JSONB — queryable by Claude via MCP.
    #[allow(clippy::too_many_arguments)]
    pub async fn insert_closed_trade(
        &self,
        tenant_id:    &str,
        symbol:       &str,
        side:         &str,
        entry_price:  f64,
        exit_price:   f64,
        size_usd:     f64,
        pnl_usd:      f64,
        pnl_pct:      f64,
        r_multiple:   f64,
        fees_usd:     f64,
        opened_at:    Option<DateTime<Utc>>,
        close_reason: &str,
        signal_json:  Option<serde_json::Value>,
    ) -> Result<()> {
        sqlx::query(
            r#"INSERT INTO closed_trades (
                tenant_id, symbol, side,
                entry_price, exit_price, size_usd,
                pnl_usd, pnl_pct, r_multiple, fees_usd,
                opened_at, close_reason, signal_contrib
            ) VALUES (
                $1::uuid, $2, $3,
                $4, $5, $6,
                $7, $8, $9, $10,
                $11, $12, $13
            )"#
        )
        .bind(tenant_id).bind(symbol).bind(side)
        .bind(entry_price).bind(exit_price).bind(size_usd)
        .bind(pnl_usd).bind(pnl_pct).bind(r_multiple).bind(fees_usd)
        .bind(opened_at).bind(close_reason).bind(signal_json)
        .execute(&self.pool)
        .await
        .with_context(|| format!("insert_closed_trade: tenant={tenant_id} {symbol}"))?;
        Ok(())
    }

    // ── Tenants ─────────────────────────────────────────────────────────────────

    /// Upsert a tenant (called at startup and on Privy login).
    pub async fn upsert_tenant(
        &self,
        id:              &str,
        privy_did:       Option<&str>,
        wallet_address:  Option<&str>,
        display_name:    Option<&str>,
        tier:            &str,
        initial_capital: f64,
    ) -> Result<()> {
        sqlx::query(
            r#"INSERT INTO tenants (id, privy_did, wallet_address, display_name, tier, initial_capital)
               VALUES ($1::uuid, $2, $3, $4, $5, $6)
               ON CONFLICT (id) DO UPDATE SET
                   privy_did       = EXCLUDED.privy_did,
                   wallet_address  = EXCLUDED.wallet_address,
                   display_name    = EXCLUDED.display_name,
                   tier            = EXCLUDED.tier,
                   initial_capital = EXCLUDED.initial_capital"#
        )
        .bind(id).bind(privy_did).bind(wallet_address)
        .bind(display_name).bind(tier).bind(initial_capital)
        .execute(&self.pool)
        .await
        .with_context(|| format!("upsert_tenant: {id}"))?;
        Ok(())
    }

    // ── Signal weights ──────────────────────────────────────────────────────────

    /// Persist updated signal weights. Called by the learner after every close.
    pub async fn upsert_signal_weights(
        &self,
        tenant_id: &str,
        weights:   &serde_json::Value,
    ) -> Result<()> {
        sqlx::query(
            r#"INSERT INTO signal_weights (tenant_id, weights, updated_at)
               VALUES ($1::uuid, $2, now())
               ON CONFLICT (tenant_id) DO UPDATE SET
                   weights    = EXCLUDED.weights,
                   updated_at = now()"#
        )
        .bind(tenant_id)
        .bind(weights)
        .execute(&self.pool)
        .await
        .with_context(|| format!("upsert_signal_weights: {tenant_id}"))?;
        Ok(())
    }

    /// Load signal weights for a tenant (called at startup to restore learning).
    pub async fn load_signal_weights(
        &self,
        tenant_id: &str,
    ) -> Result<Option<serde_json::Value>> {
        let row = sqlx::query!(
            "SELECT weights FROM signal_weights WHERE tenant_id = $1::uuid",
            tenant_id,
        )
        .fetch_optional(&self.pool)
        .await
        .with_context(|| format!("load_signal_weights: {tenant_id}"))?;
        Ok(row.map(|r| r.weights))
    }

    // ── Maintenance ─────────────────────────────────────────────────────────────

    /// Prune old equity snapshots and update query-planner statistics.
    /// Designed to be called from a background task every hour.
    pub async fn run_maintenance(&self) -> Result<()> {
        sqlx::query("SELECT prune_equity_snapshots()")
            .execute(&self.pool)
            .await
            .context("equity snapshot pruning failed")?;

        // ANALYZE tells the query planner about current row distributions.
        sqlx::query("ANALYZE equity_snapshots, aum_snapshots, closed_trades")
            .execute(&self.pool)
            .await
            .context("ANALYZE failed")?;

        log::debug!("🗄  DB maintenance complete");
        Ok(())
    }
}

// ─────────────────────────────────────────────────────────────────────────────
//  Ollama — local LLM inference
//
//  Ollama runs on the VPS at http://localhost:11434 (or OLLAMA_BASE_URL).
//  Use case: send daily trade summaries to llama3.2 / mistral for natural-
//  language analysis without Anthropic API costs.  Works offline.
//
//  Future roadmap:
//    1. Query closed_trades from DB → build context string
//    2. POST to Ollama /api/generate
//    3. Store result in ai_analyses table
//    4. Surface in admin "AI Insights" panel
// ─────────────────────────────────────────────────────────────────────────────

/// Send a prompt to a local Ollama instance and return the response text.
/// Returns an error (non-fatal) if Ollama is not running.
pub async fn query_ollama(
    base_url: &str,
    model:    &str,
    prompt:   &str,
) -> Result<String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .build()
        .context("Failed to build reqwest client for Ollama")?;

    let resp = client
        .post(format!("{base_url}/api/generate"))
        .json(&serde_json::json!({
            "model":  model,
            "prompt": prompt,
            "stream": false,
        }))
        .send()
        .await
        .context("Ollama POST failed — is `ollama serve` running?")?;

    let json: serde_json::Value = resp.json().await.context("Ollama response parse failed")?;
    Ok(json["response"].as_str().unwrap_or("").to_string())
}
