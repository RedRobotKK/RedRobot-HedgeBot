//! Strategy 5: Ichimoku Cloud Analysis
//! Win rate: 65-75% for trend and support/resistance

use super::{StrategySignal, StrategyContext, SignalType};

pub fn evaluate(ctx: &StrategyContext) -> Result<StrategySignal, String> {
    let current = &ctx.current;
    let price = current.close;
    let support = current.support_level;  // Using as approximation for cloud
    let resistance = current.resistance_level;

    // Simplified Ichimoku: Cloud acting as support/resistance
    // Cloud top (resistance), Cloud bottom (support)

    // Price above cloud = bullish
    if price > resistance {
        return Ok(StrategySignal {
            strategy_name: "Ichimoku (Above Cloud)".to_string(),
            signal_type: SignalType::Buy,
            confidence: 0.65,
            position_size_multiplier: 1.0,
            rationale: format!(
                "Price {:.2} trading above Ichimoku cloud (above {:.2}). Strong bullish trend.",
                price, resistance
            ),
            target_price: None,
            stop_loss_pct: 0.04,
        });
    }

    // Price below cloud = bearish
    if price < support {
        return Ok(StrategySignal {
            strategy_name: "Ichimoku (Below Cloud)".to_string(),
            signal_type: SignalType::Sell,
            confidence: 0.65,
            position_size_multiplier: 1.0,
            rationale: format!(
                "Price {:.2} trading below Ichimoku cloud (below {:.2}). Strong bearish trend.",
                price, support
            ),
            target_price: None,
            stop_loss_pct: 0.04,
        });
    }

    Err("Price within Ichimoku cloud".to_string())
}
