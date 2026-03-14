//! Perpetual futures funding rate module.
//!
//! Funding rate = the periodic payment exchanged between longs and shorts on
//! perpetual futures, settled every 8 hours.  It is a direct, real-time measure
//! of market leverage and crowd positioning — and therefore a powerful
//! **contrarian** signal:
//!
//! | 8h rate       | Market state           | Signal          |
//! |---------------|------------------------|-----------------|
//! | > +0.10 %     | Extreme long crowding  | Strong BEAR     |
//! | +0.05–0.10 %  | Elevated long crowding | Moderate BEAR   |
//! | +0.02–0.05 %  | Mild long bias         | Slight BEAR     |
//! | ±0.02 %       | Neutral                | No signal       |
//! | −0.02–−0.05 % | Mild short bias        | Slight BULL     |
//! | −0.05–−0.10 % | Elevated short crowd   | Moderate BULL   |
//! | < −0.10 %     | Extreme short crowding | Strong BULL     |
//!
//! High positive funding → longs are overcrowded and paying shorts to stay open.
//! When long crowding unwinds, prices fall quickly as stops are hit in cascade.
//!
//! Source: Binance USDT-M Futures (free, no auth required).
//!   `GET https://fapi.binance.com/fapi/v1/premiumIndex`
//!   Returns all symbols in one call; cached for 3 minutes.

use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// Refresh every 3 minutes.  Funding changes every 8 h but can spike quickly
/// and the Binance endpoint is free with generous rate limits.
const CACHE_TTL: Duration = Duration::from_secs(180);

// ─────────────────────────── Public types ────────────────────────────────────

/// Funding rate snapshot for a single perpetual symbol.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FundingData {
    pub symbol:       String,
    /// Last realised 8-hour funding rate (e.g. 0.0001 = 0.01 %).
    /// Positive = longs pay shorts.  Negative = shorts pay longs.
    pub funding_rate: f64,
    /// Change in funding rate vs the previous cache refresh cycle.
    /// Positive = funding rising (longs becoming more crowded / expensive).
    /// Negative = funding falling (de-levering, shorts building up).
    /// Zero      = first observation or no meaningful change.
    pub funding_delta: f64,
}

impl FundingData {
    /// Annualised rate as a percentage (rate × 3 payments/day × 365 days × 100).
    #[allow(dead_code)]
    pub fn annualised_pct(&self) -> f64 {
        self.funding_rate * 3.0 * 365.0 * 100.0
    }

    /// Contrarian signal strength in **[−1.0, +1.0]**.
    ///
    /// * Positive → bullish lean (shorts overcrowded → squeeze risk).
    /// * Negative → bearish lean (longs overcrowded → liquidation risk).
    /// * Zero     → neutral / below noise threshold.
    pub fn signal_strength(&self) -> f64 {
        let r = self.funding_rate;
        // Thresholds expressed in 8h rate units:
        //   0.0010 = 0.10 %  (extreme)
        //   0.0005 = 0.05 %  (elevated)
        //   0.0002 = 0.02 %  (mild — outer edge of neutral band)
        if      r >  0.0010 { -1.00 }  // extreme long crowding  → strong bear lean
        else if r >  0.0005 { -0.65 }  // elevated long crowding → moderate bear lean
        else if r >  0.0002 { -0.30 }  // mild long bias         → slight bear lean
        else if r < -0.0010 {  1.00 }  // extreme short crowding → strong bull lean
        else if r < -0.0005 {  0.65 }  // elevated short crowding → moderate bull lean
        else if r < -0.0002 {  0.30 }  // mild short bias         → slight bull lean
        else                {  0.00 }  // neutral band            → no signal
    }

    /// True when the rate is outside the neutral ±0.02 % band.
    pub fn is_significant(&self) -> bool {
        self.funding_rate.abs() > 0.0002
    }

    /// Emoji indicator for dashboard display.
    pub fn emoji(&self) -> &'static str {
        let r = self.funding_rate;
        if      r >  0.0005 { "🔴" }  // elevated longs → bearish
        else if r < -0.0005 { "🟢" }  // elevated shorts → bullish
        else                { "🟡" }  // neutral
    }
}

// ─────────────────────────── Binance API response shape ──────────────────────

#[derive(Deserialize)]
struct PremiumIndexItem {
    symbol: String,
    #[serde(rename = "lastFundingRate")]
    last_funding_rate: String,
}

// ─────────────────────────── Cache ───────────────────────────────────────────

struct CacheInner {
    data:       HashMap<String, FundingData>,
    /// Rates from the previous refresh — used to compute `funding_delta`.
    prev_rates: HashMap<String, f64>,
    last_fetch: Option<Instant>,
}

/// Thread-safe, auto-refreshing funding rate cache.
/// Clone the `Arc` freely — one instance per bot.
pub struct FundingCache {
    client: Client,
    inner:  RwLock<CacheInner>,
}

pub type SharedFunding = Arc<FundingCache>;

impl FundingCache {
    pub fn new() -> SharedFunding {
        Arc::new(FundingCache {
            client: Client::builder()
                .timeout(Duration::from_secs(8))
                .build()
                .unwrap_or_default(),
            inner: RwLock::new(CacheInner {
                data:       HashMap::new(),
                prev_rates: HashMap::new(),
                last_fetch: None,
            }),
        })
    }

    /// Look up funding data for `symbol` (Hyperliquid short form: "ETH", "SOL").
    /// Transparently refreshes when the TTL has expired.
    /// Returns `None` if the symbol is not listed on Binance USDT-M futures.
    pub async fn get(&self, symbol: &str) -> Option<FundingData> {
        // Fast path: cache is warm
        {
            let r = self.inner.read().await;
            if r.last_fetch.map(|t| t.elapsed() < CACHE_TTL).unwrap_or(false) {
                return r.data.get(symbol).cloned();
            }
        }
        // Cache is stale — snapshot current rates as prev before refreshing
        let prev_rates: HashMap<String, f64> = {
            let r = self.inner.read().await;
            r.data.iter().map(|(k, v)| (k.clone(), v.funding_rate)).collect()
        };

        match self.fetch_all(&prev_rates).await {
            Ok(map) => {
                let result = map.get(symbol).cloned();
                let mut w  = self.inner.write().await;
                w.prev_rates = prev_rates;
                w.data       = map;
                w.last_fetch = Some(Instant::now());
                result
            }
            Err(e) => {
                log::warn!("💰 Funding fetch error: {} — using stale cache", e);
                self.inner.read().await.data.get(symbol).cloned()
            }
        }
    }

    /// Pre-warm the cache at startup (avoids first-cycle fetch latency).
    /// First warm has no previous rates, so `funding_delta` will be 0.0.
    pub async fn warm(&self) {
        let empty_prev: HashMap<String, f64> = HashMap::new();
        match self.fetch_all(&empty_prev).await {
            Ok(map) => {
                log::info!(
                    "💰 Funding rates: pre-warmed {} USDT perps  \
                     (BTC={:+.4}%  ETH={:+.4}%)",
                    map.len(),
                    map.get("BTC").map(|d| d.funding_rate * 100.0).unwrap_or(0.0),
                    map.get("ETH").map(|d| d.funding_rate * 100.0).unwrap_or(0.0),
                );
                let mut w  = self.inner.write().await;
                w.data       = map;
                w.last_fetch = Some(Instant::now());
            }
            Err(e) => log::warn!("💰 Funding warm-up failed: {}", e),
        }
    }

    // ── Private ───────────────────────────────────────────────────────────────

    /// Fetch the full Binance `premiumIndex` snapshot (all symbols, one call).
    /// `prev` provides the rates from the last refresh for delta computation.
    async fn fetch_all(&self, prev: &HashMap<String, f64>) -> Result<HashMap<String, FundingData>> {
        let resp = self.client
            .get("https://fapi.binance.com/fapi/v1/premiumIndex")
            .send()
            .await?;

        if !resp.status().is_success() {
            anyhow::bail!("Binance premiumIndex HTTP {}", resp.status());
        }

        let items: Vec<PremiumIndexItem> = resp.json().await?;
        let mut map = HashMap::with_capacity(items.len());

        for item in items {
            // Only keep USDT-margined perpetuals (skip coin-margined like BTCUSD_PERP)
            if !item.symbol.ends_with("USDT") { continue; }

            let rate: f64 = item.last_funding_rate.parse().unwrap_or(0.0);

            // Strip "USDT" suffix → short symbol form used throughout the bot
            // e.g. "ETHUSDT" → "ETH", "SOLUSDT" → "SOL"
            let sym = item.symbol.trim_end_matches("USDT").to_string();

            // Delta vs previous cycle (0 on first observation)
            let delta = prev.get(&sym).map(|&p| rate - p).unwrap_or(0.0);

            map.insert(sym.clone(), FundingData {
                symbol:        sym,
                funding_rate:  rate,
                funding_delta: delta,
            });
        }

        log::info!(
            "💰 Funding rates: {} USDT perps  (BTC={:+.4}%  ETH={:+.4}%  SOL={:+.4}%)",
            map.len(),
            map.get("BTC").map(|d| d.funding_rate * 100.0).unwrap_or(0.0),
            map.get("ETH").map(|d| d.funding_rate * 100.0).unwrap_or(0.0),
            map.get("SOL").map(|d| d.funding_rate * 100.0).unwrap_or(0.0),
        );
        Ok(map)
    }
}
