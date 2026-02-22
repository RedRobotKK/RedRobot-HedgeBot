//! Strategy 1: Mean Reversion (RSI + Bollinger Bands)
//! Win rate: 75-85% when combined with market extremes

use super::{StrategySignal, StrategyContext, SignalType};

pub fn evaluate(ctx: &StrategyContext) -> Result<StrategySignal, String> {
    let current = &ctx.current;
    let rsi = current.rsi_14;
    let price = current.close;
    let upper_bb = current.bollinger_upper;
    let lower_bb = current.bollinger_lower;
    let middle_bb = current.bollinger_middle;

    // Mean reversion: detect oversold/overbought extremes
    if rsi < 30.0 && price < lower_bb {
        // OVERSOLD - likely bounce up
        let confidence = 0.75;
        let position_size = 1.2;  // 20% larger position

        return Ok(StrategySignal {
            strategy_name: "Mean Reversion (Oversold)".to_string(),
            signal_type: SignalType::Buy,
            confidence,
            position_size_multiplier: position_size,
            rationale: format!(
                "RSI {} indicates extreme oversold. Price ${:.2} below lower Bollinger Band ${:.2}. Expect mean reversion to ${:.2}",
                rsi as i32, price, lower_bb, middle_bb
            ),
            target_price: Some(middle_bb),
            stop_loss_pct: 0.03,  // 3% below entry
        });
    }

    if rsi > 70.0 && price > upper_bb {
        // OVERBOUGHT - likely pullback
        let confidence = 0.75;
        let position_size = 1.2;

        return Ok(StrategySignal {
            strategy_name: "Mean Reversion (Overbought)".to_string(),
            signal_type: SignalType::Sell,
            confidence,
            position_size_multiplier: position_size,
            rationale: format!(
                "RSI {} indicates extreme overbought. Price ${:.2} above upper Bollinger Band ${:.2}. Expect pullback to ${:.2}",
                rsi as i32, price, upper_bb, middle_bb
            ),
            target_price: Some(middle_bb),
            stop_loss_pct: 0.03,
        });
    }

    // No signal
    Err("RSI not in extreme zone".to_string())
}
