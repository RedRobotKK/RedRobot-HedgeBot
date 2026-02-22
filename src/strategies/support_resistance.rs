//! Strategy 4: Support/Resistance Bounce
//! Win rate: 70-80% when price bounces off clear levels

use super::{StrategySignal, StrategyContext, SignalType};

pub fn evaluate(ctx: &StrategyContext) -> Result<StrategySignal, String> {
    let current = &ctx.current;
    let price = current.close;
    let support = current.support_level;
    let resistance = current.resistance_level;
    let atr = current.atr_14;

    // Bounce off support
    let distance_to_support = (price - support).abs();
    if distance_to_support < atr * 0.5 && price > support {
        // Price just bounced off support
        return Ok(StrategySignal {
            strategy_name: "Support Bounce".to_string(),
            signal_type: SignalType::Buy,
            confidence: 0.70,
            position_size_multiplier: 1.0,
            rationale: format!(
                "Price {:.2} bounced off support at {:.2}. Strong reversal zone. Target resistance at {:.2}",
                price, support, resistance
            ),
            target_price: Some(resistance),
            stop_loss_pct: 0.02,
        });
    }

    // Bounce off resistance
    let distance_to_resistance = (price - resistance).abs();
    if distance_to_resistance < atr * 0.5 && price < resistance {
        // Price just bounced off resistance
        return Ok(StrategySignal {
            strategy_name: "Resistance Bounce".to_string(),
            signal_type: SignalType::Sell,
            confidence: 0.70,
            position_size_multiplier: 1.0,
            rationale: format!(
                "Price {:.2} bounced off resistance at {:.2}. Strong reversal zone. Target support at {:.2}",
                price, resistance, support
            ),
            target_price: Some(support),
            stop_loss_pct: 0.02,
        });
    }

    Err("No support/resistance bounce".to_string())
}
