//! Strategy 6: Stochastic Oscillator Crossover
//! Win rate: 65-75% for overbought/oversold detection

use super::{StrategySignal, StrategyContext, SignalType};

pub fn evaluate(ctx: &StrategyContext) -> Result<StrategySignal, String> {
    let current = &ctx.current;
    let previous = ctx.previous.as_ref().ok_or("Need previous data")?;
    let stoch_k = current.stoch_k;
    let stoch_d = current.stoch_d;

    // K crossing above D in oversold territory = bullish
    if previous.stoch_k <= previous.stoch_d && stoch_k > stoch_d && stoch_k < 30.0 {
        return Ok(StrategySignal {
            strategy_name: "Stochastic (Bullish Crossover)".to_string(),
            signal_type: SignalType::Buy,
            confidence: 0.70,
            position_size_multiplier: 1.0,
            rationale: format!(
                "Stochastic K {:.1} crossing above D {:.1} in oversold territory (<30). Bullish momentum shift.",
                stoch_k, stoch_d
            ),
            target_price: Some(current.bollinger_upper),
            stop_loss_pct: 0.03,
        });
    }

    // K crossing below D in overbought territory = bearish
    if previous.stoch_k >= previous.stoch_d && stoch_k < stoch_d && stoch_k > 70.0 {
        return Ok(StrategySignal {
            strategy_name: "Stochastic (Bearish Crossover)".to_string(),
            signal_type: SignalType::Sell,
            confidence: 0.70,
            position_size_multiplier: 1.0,
            rationale: format!(
                "Stochastic K {:.1} crossing below D {:.1} in overbought territory (>70). Bearish momentum shift.",
                stoch_k, stoch_d
            ),
            target_price: Some(current.bollinger_lower),
            stop_loss_pct: 0.03,
        });
    }

    Err("No Stochastic crossover".to_string())
}
