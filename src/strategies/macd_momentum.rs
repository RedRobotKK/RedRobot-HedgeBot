//! Strategy 2: MACD Momentum (Moving Average Convergence Divergence)
//! Win rate: 60-70% for trend following

use super::{StrategySignal, StrategyContext, SignalType};

pub fn evaluate(ctx: &StrategyContext) -> Result<StrategySignal, String> {
    let current = &ctx.current;
    let macd = current.macd;
    let signal_line = current.macd_signal;
    let histogram = current.macd_histogram;
    let previous = ctx.previous.as_ref().ok_or("Need previous data")?;

    // MACD above signal line = uptrend
    if macd > signal_line && macd > 0.0 {
        let histogram_increasing = histogram > previous.macd_histogram;
        let confidence = if histogram_increasing { 0.75 } else { 0.60 };

        return Ok(StrategySignal {
            strategy_name: "MACD Momentum (Uptrend)".to_string(),
            signal_type: SignalType::Buy,
            confidence,
            position_size_multiplier: 1.0,
            rationale: format!(
                "MACD {:.4} above signal {:.4} indicates uptrend momentum. Histogram {}. Trail stop recommended.",
                macd, signal_line,
                if histogram_increasing { "increasing" } else { "decreasing" }
            ),
            target_price: None,  // Let trend run
            stop_loss_pct: 0.05,
        });
    }

    // MACD below signal line = downtrend
    if macd < signal_line && macd < 0.0 {
        let histogram_decreasing = histogram < previous.macd_histogram;
        let confidence = if histogram_decreasing { 0.75 } else { 0.60 };

        return Ok(StrategySignal {
            strategy_name: "MACD Momentum (Downtrend)".to_string(),
            signal_type: SignalType::Sell,
            confidence,
            position_size_multiplier: 1.0,
            rationale: format!(
                "MACD {:.4} below signal {:.4} indicates downtrend momentum. Histogram {}. Trail stop recommended.",
                macd, signal_line,
                if histogram_decreasing { "decreasing" } else { "increasing" }
            ),
            target_price: None,  // Let trend run
            stop_loss_pct: 0.05,
        });
    }

    Err("MACD not in clear momentum".to_string())
}
