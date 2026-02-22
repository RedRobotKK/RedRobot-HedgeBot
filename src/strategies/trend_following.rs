//! Strategy 8: Trend Following (ADX-based)
//! Win rate: 55-65% but long-term profitable with compound gains

use super::{StrategySignal, StrategyContext, SignalType};

pub fn evaluate(ctx: &StrategyContext) -> Result<StrategySignal, String> {
    let current = &ctx.current;
    let price = current.close;
    let adx = current.adx;
    let previous = ctx.previous.as_ref().ok_or("Need previous data")?;

    // ADX > 25 indicates strong trend
    if adx > 25.0 {
        // Determine direction from price vs moving average (approximated by support/resistance)
        if price > current.resistance_level && price > previous.close {
            // Uptrend
            return Ok(StrategySignal {
                strategy_name: "Trend Following (Strong Uptrend)".to_string(),
                signal_type: SignalType::Buy,
                confidence: 0.65,
                position_size_multiplier: 1.0,
                rationale: format!(
                    "ADX {:.1} indicates strong uptrend. Price {:.2} above resistance {:.2}. Let trend run with trailing stop.",
                    adx, price, current.resistance_level
                ),
                target_price: None,
                stop_loss_pct: 0.05,
            });
        } else if price < current.support_level && price < previous.close {
            // Downtrend
            return Ok(StrategySignal {
                strategy_name: "Trend Following (Strong Downtrend)".to_string(),
                signal_type: SignalType::Sell,
                confidence: 0.65,
                position_size_multiplier: 1.0,
                rationale: format!(
                    "ADX {:.1} indicates strong downtrend. Price {:.2} below support {:.2}. Let trend run with trailing stop.",
                    adx, price, current.support_level
                ),
                target_price: None,
                stop_loss_pct: 0.05,
            });
        }
    }

    Err("ADX not indicating strong trend".to_string())
}
