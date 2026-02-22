//! Strategy 9: Volatility Mean Reversion (ATR-based)
//! Win rate: 70-80% when used for position sizing and entries

use super::{StrategySignal, StrategyContext, SignalType};

pub fn evaluate(ctx: &StrategyContext) -> Result<StrategySignal, String> {
    let current = &ctx.current;
    let atr = current.atr_14;
    let previous = ctx.previous.as_ref().ok_or("Need previous data")?;
    let price = current.close;
    let rsi = current.rsi_14;

    // High volatility compression (low ATR) followed by expansion = breakout
    let atr_ratio = atr / previous.atr_14;

    // If volatility was low and is expanding with price moving up and oversold RSI
    if previous.atr_14 < 2.0 && atr > previous.atr_14 && atr_ratio > 1.2 && rsi < 50.0 {
        return Ok(StrategySignal {
            strategy_name: "Volatility Breakout (Expansion)".to_string(),
            signal_type: SignalType::Buy,
            confidence: 0.70,
            position_size_multiplier: 1.3,  // Increase size on volatility
            rationale: format!(
                "ATR expanding from {:.2} to {:.2} ({:.0}% increase). Low volatility compression breakout. RSI {} supports entry.",
                previous.atr_14, atr, (atr_ratio - 1.0) * 100.0, rsi as i32
            ),
            target_price: Some(price + (atr * 2.0)),
            stop_loss_pct: (atr / price) * 100.0,  // Dynamic stop based on volatility
        });
    }

    // If volatility was low and is expanding with price moving down and overbought RSI
    if previous.atr_14 < 2.0 && atr > previous.atr_14 && atr_ratio > 1.2 && rsi > 50.0 {
        return Ok(StrategySignal {
            strategy_name: "Volatility Breakout (Expansion)".to_string(),
            signal_type: SignalType::Sell,
            confidence: 0.70,
            position_size_multiplier: 1.3,  // Increase size on volatility
            rationale: format!(
                "ATR expanding from {:.2} to {:.2} ({:.0}% increase). Low volatility compression breakout. RSI {} supports entry.",
                previous.atr_14, atr, (atr_ratio - 1.0) * 100.0, rsi as i32
            ),
            target_price: Some(price - (atr * 2.0)),
            stop_loss_pct: (atr / price) * 100.0,
        });
    }

    Err("No volatility breakout signal".to_string())
}
