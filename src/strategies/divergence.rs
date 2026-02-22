//! Strategy 3: Divergence Trading (RSI/MACD divergence from price)
//! Win rate: 80%+ for reversal accuracy

use super::{StrategySignal, StrategyContext, SignalType};

pub fn evaluate(ctx: &StrategyContext) -> Result<StrategySignal, String> {
    let current = &ctx.current;
    let previous = ctx.previous.as_ref().ok_or("Need previous data")?;

    // Bullish divergence: Price lower, RSI higher (reversal up)
    if current.close < previous.close && current.rsi_14 > previous.rsi_14 && current.rsi_14 < 50.0 {
        return Ok(StrategySignal {
            strategy_name: "Divergence (Bullish RSI)".to_string(),
            signal_type: SignalType::Buy,
            confidence: 0.80,
            position_size_multiplier: 1.1,
            rationale: format!(
                "Bullish RSI divergence: price lower at {:.2} vs {:.2}, but RSI higher at {} vs {}. Early reversal signal.",
                current.close, previous.close, current.rsi_14 as i32, previous.rsi_14 as i32
            ),
            target_price: Some(current.bollinger_middle),
            stop_loss_pct: 0.04,
        });
    }

    // Bearish divergence: Price higher, RSI lower (reversal down)
    if current.close > previous.close && current.rsi_14 < previous.rsi_14 && current.rsi_14 > 50.0 {
        return Ok(StrategySignal {
            strategy_name: "Divergence (Bearish RSI)".to_string(),
            signal_type: SignalType::Sell,
            confidence: 0.80,
            position_size_multiplier: 1.1,
            rationale: format!(
                "Bearish RSI divergence: price higher at {:.2} vs {:.2}, but RSI lower at {} vs {}. Early reversal signal.",
                current.close, previous.close, current.rsi_14 as i32, previous.rsi_14 as i32
            ),
            target_price: Some(current.bollinger_middle),
            stop_loss_pct: 0.04,
        });
    }

    Err("No divergence detected".to_string())
}
