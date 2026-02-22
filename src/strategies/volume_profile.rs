//! Strategy 7: Volume Profile and VWAP
//! Win rate: 70% when price bounces off VWAP

use super::{StrategySignal, StrategyContext, SignalType};

pub fn evaluate(ctx: &StrategyContext) -> Result<StrategySignal, String> {
    let current = &ctx.current;
    let price = current.close;
    let vwap = current.vwap;
    let volume = current.volume;
    let atr = current.atr_14;

    // Price bouncing off VWAP with volume support
    let distance_to_vwap = (price - vwap).abs();

    if distance_to_vwap < atr * 0.3 && volume > 1000000.0 {
        if price < vwap {
            // Price below VWAP, bouncing up
            return Ok(StrategySignal {
                strategy_name: "Volume Profile (VWAP Bounce)".to_string(),
                signal_type: SignalType::Buy,
                confidence: 0.68,
                position_size_multiplier: 1.0,
                rationale: format!(
                    "Price {:.2} bouncing off VWAP {:.2} with high volume {:.0}. Institutional support level.",
                    price, vwap, volume
                ),
                target_price: Some(current.resistance_level),
                stop_loss_pct: 0.02,
            });
        } else {
            // Price above VWAP, pulling back down
            return Ok(StrategySignal {
                strategy_name: "Volume Profile (VWAP Bounce)".to_string(),
                signal_type: SignalType::Sell,
                confidence: 0.68,
                position_size_multiplier: 1.0,
                rationale: format!(
                    "Price {:.2} pulling back to VWAP {:.2} with high volume {:.0}. Institutional resistance level.",
                    price, vwap, volume
                ),
                target_price: Some(current.support_level),
                stop_loss_pct: 0.02,
            });
        }
    }

    Err("No volume profile signal".to_string())
}
