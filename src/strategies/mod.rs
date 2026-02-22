//! 🎯 All 9 Wall Street Quant Technical Strategies
//! Each strategy can be used independently or combined for multi-signal confluence

pub mod mean_reversion;
pub mod macd_momentum;
pub mod divergence;
pub mod support_resistance;
pub mod ichimoku;
pub mod stochastic;
pub mod volume_profile;
pub mod trend_following;
pub mod volatility_mean_reversion;

use serde::{Deserialize, Serialize};

/// Strategy execution result with performance tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategySignal {
    pub strategy_name: String,
    pub signal_type: SignalType,
    pub confidence: f64,  // 0.0-1.0
    pub position_size_multiplier: f64,  // 1.0 = normal, 1.5 = 50% larger
    pub rationale: String,
    pub target_price: Option<f64>,
    pub stop_loss_pct: f64,  // percentage below entry
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SignalType {
    StrongBuy,
    Buy,
    Neutral,
    Sell,
    StrongSell,
}

impl SignalType {
    pub fn direction(&self) -> f64 {
        match self {
            SignalType::StrongBuy => 1.0,
            SignalType::Buy => 0.5,
            SignalType::Neutral => 0.0,
            SignalType::Sell => -0.5,
            SignalType::StrongSell => -1.0,
        }
    }
}

/// Market data snapshot for strategy analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketSnapshot {
    pub timestamp: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub rsi_14: f64,
    pub rsi_7: f64,
    pub macd: f64,
    pub macd_signal: f64,
    pub macd_histogram: f64,
    pub bollinger_upper: f64,
    pub bollinger_middle: f64,
    pub bollinger_lower: f64,
    pub atr_14: f64,
    pub stoch_k: f64,
    pub stoch_d: f64,
    pub support_level: f64,
    pub resistance_level: f64,
    pub vwap: f64,
    pub adx: f64,  // Trend strength (0-100)
    pub fear_greed_index: Option<u32>,  // 0-100
}

/// Strategy evaluation context
#[derive(Debug, Clone)]
pub struct StrategyContext {
    pub current: MarketSnapshot,
    pub previous: Option<MarketSnapshot>,
    pub cex_imbalance_ratio: f64,  // bid/ask
    pub cex_signal_type: SignalType,  // Order flow direction
    pub portfolio_equity: f64,
    pub portfolio_drawdown_pct: f64,
    pub position_open: bool,
}

/// Evaluate all 9 strategies and return signals
pub fn evaluate_all_strategies(ctx: &StrategyContext) -> Vec<StrategySignal> {
    let mut signals = vec![];

    // Run each strategy
    if let Ok(signal) = mean_reversion::evaluate(&ctx) {
        signals.push(signal);
    }
    if let Ok(signal) = macd_momentum::evaluate(&ctx) {
        signals.push(signal);
    }
    if let Ok(signal) = divergence::evaluate(&ctx) {
        signals.push(signal);
    }
    if let Ok(signal) = support_resistance::evaluate(&ctx) {
        signals.push(signal);
    }
    if let Ok(signal) = ichimoku::evaluate(&ctx) {
        signals.push(signal);
    }
    if let Ok(signal) = stochastic::evaluate(&ctx) {
        signals.push(signal);
    }
    if let Ok(signal) = volume_profile::evaluate(&ctx) {
        signals.push(signal);
    }
    if let Ok(signal) = trend_following::evaluate(&ctx) {
        signals.push(signal);
    }
    if let Ok(signal) = volatility_mean_reversion::evaluate(&ctx) {
        signals.push(signal);
    }

    signals
}

/// Score multiple signals for confluence
pub fn calculate_confluence_score(signals: &[StrategySignal]) -> f64 {
    if signals.is_empty() {
        return 0.0;
    }

    let mut buy_signals = 0;
    let mut sell_signals = 0;

    for signal in signals {
        match signal.signal_type {
            SignalType::StrongBuy => buy_signals += 2,
            SignalType::Buy => buy_signals += 1,
            SignalType::StrongSell => sell_signals += 2,
            SignalType::Sell => sell_signals += 1,
            _ => {}
        }
    }

    // Base confidence from signal count
    let base = 0.65 + (signals.len() as f64 * 0.06);
    let base_capped = base.min(0.95);

    // Directional alignment bonus
    let alignment = if buy_signals > 0 && sell_signals == 0 {
        0.10  // All bullish
    } else if sell_signals > 0 && buy_signals == 0 {
        0.10  // All bearish
    } else if buy_signals > sell_signals {
        0.05  // Mostly bullish
    } else if sell_signals > buy_signals {
        0.05  // Mostly bearish
    } else {
        0.0   // Conflicted
    };

    (base_capped + alignment).min(0.95)
}
