//! Hyperliquid exchange client.
//!
//! # ⚠ STUB — Paper-mode only
//!
//! The methods in this module currently return **hardcoded mock data** and do
//! NOT make real API calls to Hyperliquid.  This is intentional: the bot
//! operates in paper-trading mode and live order execution has not yet been
//! implemented.
//!
//! ## Production TODO list
//!
//! When implementing live trading, replace the stubs with:
//!
//! 1. **`get_account()`** — POST `/info` with `{"type":"clearinghouseState","user":"<addr>"}`
//!    and parse `marginSummary.{accountValue, crossMaintenanceMarginUsed}`.
//!
//! 2. **`place_order()`** — Sign and POST to `/exchange` with an `order` action.
//!    Requires EIP-712 signing of the order struct with the wallet private key.
//!    Reference: <https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/exchange-endpoint>
//!
//! 3. **`get_positions()`** — Parse `assetPositions` from the clearing-house state
//!    response.
//!
//! 4. **`close_position()`** — Place a reduce-only market order opposite to the
//!    open position.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use crate::config::Config;
use crate::decision::Decision;
use crate::risk::Account;

// ─────────────────────────── Data types ──────────────────────────────────────

/// An open position on Hyperliquid.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub symbol:        String,
    pub size:          f64,
    pub entry_price:   f64,
    pub current_price: f64,
    pub pnl:           f64,
    pub leverage:      f64,
}

impl Position {
    /// Returns `true` when the position has moved ≥ 5% in either direction.
    ///
    /// This is a simple exit signal for live-mode position management;
    /// the paper engine uses the more sophisticated R-multiple / trailing-stop
    /// logic in `main.rs`.
    pub fn should_close(&self) -> bool {
        let pnl_pct = (self.current_price - self.entry_price) / self.entry_price;
        pnl_pct.abs() > 0.05
    }
}

// ─────────────────────────── Client ──────────────────────────────────────────

/// HTTP client for the Hyperliquid exchange API.
///
/// **All methods are currently stubs** — see the module-level doc for the
/// production implementation roadmap.
#[derive(Debug)]
pub struct HyperliquidClient {
    #[allow(dead_code)] // will be used once stubs are replaced with real calls
    client:   reqwest::Client,
    #[allow(dead_code)]
    base_url: String,
    #[allow(dead_code)]
    testnet:  bool,
}

impl HyperliquidClient {
    /// Create a new client pointed at the correct API endpoint for `config.mode`.
    pub fn new(config: &Config) -> Result<Self> {
        let base_url = match config.mode {
            crate::config::Mode::Testnet => "https://api.hyperliquid-testnet.xyz".to_string(),
            crate::config::Mode::Mainnet => "https://api.hyperliquid.xyz".to_string(),
            crate::config::Mode::Paper   => "https://api.hyperliquid.xyz".to_string(),
        };

        Ok(HyperliquidClient {
            client:   reqwest::Client::new(),
            base_url,
            testnet:  matches!(config.mode, crate::config::Mode::Testnet),
        })
    }

    // ─────────────────── STUBS (replace for live trading) ────────────────────

    /// STUB — Returns a hardcoded healthy account with $100 equity.
    ///
    /// Production: POST `/info` with `clearinghouseState` to get live values.
    pub async fn get_account(&self) -> Result<Account> {
        // TODO: replace with real Hyperliquid clearinghouseState call
        Ok(Account {
            equity:           100.0,
            margin:            23.0,
            health_factor:      4.2,
            daily_pnl:          2.50,
            daily_loss_limit:  30.0,
        })
    }

    /// STUB — Logs the order and returns a random UUID without sending to exchange.
    ///
    /// Production: sign the order with EIP-712 and POST to `/exchange`.
    pub async fn place_order(&self, decision: &Decision) -> Result<String> {
        if decision.action == "SKIP" {
            return Err(anyhow::anyhow!("Decision is SKIP — nothing to place"));
        }
        // TODO: replace with real signed order submission
        let order_id = uuid::Uuid::new_v4().to_string();
        log::info!("📍 [STUB] Order placed: {}", order_id);
        Ok(order_id)
    }

    /// STUB — Always returns an empty position list.
    ///
    /// Production: parse `assetPositions` from the `clearinghouseState` response.
    pub async fn get_positions(&self) -> Result<Vec<Position>> {
        // TODO: replace with real position fetch
        Ok(vec![])
    }

    /// STUB — Logs the close request and returns a random UUID.
    ///
    /// Production: place a reduce-only market order opposite to `position.size`.
    pub async fn close_position(&self, position: &Position) -> Result<String> {
        // TODO: replace with real reduce-only order
        let order_id = uuid::Uuid::new_v4().to_string();
        log::info!("🔒 [STUB] Position closed: {} ({})", order_id, position.symbol);
        Ok(order_id)
    }
}
