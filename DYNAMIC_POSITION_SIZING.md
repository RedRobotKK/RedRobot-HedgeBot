# 📐 Dynamic Position Sizing: Pain vs Reward Framework

## The Framework

Your approach: **Position size = f(support distance, technical setup, acceptable risk)**

Instead of fixed 25% per entry, calculate position size dynamically based on:

1. **Support Level** (HARD FLOOR - don't trade below this)
2. **Technical Setup** (RSI, MACD, Bollinger - entry timing)
3. **Risk/Reward Ratio** (pain vs reward)
4. **Capital at Risk** (max 5% loss per trade)

---

## Real Example: SOL at $82

### Market Setup
```
Current price:    $82
Support level:    $60 (hard floor)
Resistance level: $85

Runway to support: $22 (26.8% of price)
This is your "pain" - how much you can lose if wrong
```

### Technical Setup
```
RSI:              28 (oversold, very bullish)
MACD:             Above signal line (bullish momentum)
Bollinger Bands:  Price near lower band (extreme)
Setup confidence: HIGH (multiple technicals aligned)
Expected move:    +3% (strong technical setup)
```

### Dynamic Position Sizing Calculation

```
Step 1: Define acceptable risk per trade
  Max loss per trade: 5% of $1,000 capital = $50

Step 2: Calculate position size based on support distance
  Support is $22 below current price
  If position drops to support: $22 loss per unit of position

  To stay within $50 loss limit:
    Position size = $50 / ($22/$82)
    Position size = $50 / 0.268
    Position size = $186.57 (18.6% of capital)

Step 3: Cap at reasonable max (25% per trade)
  18.6% < 25%, so use 18.6%

Step 4: Calculate expected profit and risk/reward
  Position size: $186.57 @ $82
  Expected move: +3% = +$5.60 profit
  Max loss if support breaks: -$50
  Risk/Reward ratio: $5.60 / $50 = 0.11:1 ❌ NOT GOOD
```

**Decision: Skip Entry 1 at $82**

Reason: Not enough expected profit vs risk. Expected 3% move ($5.60) doesn't justify $50 risk (only 0.11:1 ratio).

---

## Wait for Better Setup: Price Drops to $75

```
New position:
Current price:    $75
Support level:    $60 (unchanged)
Runway:           $15 (20% of price)

Technical update:
RSI:              18 (even more oversold!)
MACD:             Still bullish
Bollinger:        Touching lower band
Expected move:    +4% (even stronger setup now)

Position calculation:
  Max loss: $50 (same)
  Position size = $50 / ($15/$75) = $50 / 0.20 = $250 (25% of capital)

Expected profit: $250 × 4% = +$10
Risk/Reward: $10 / $50 = 0.20:1 ❌ STILL NOT GOOD
```

**Decision: Still skip, or take TINY position**

The technicals are better, but runway to support has shrunk. The math doesn't work.

---

## Price Tests Support at $62 - NOW IT WORKS

```
Current price:    $62
Support level:    $60
Runway:           $2 (only 3.2% to support)

Technical setup:
RSI:              12 (extreme capitulation)
MACD:             Strong bullish cross
Bollinger:        Way below lower band
Expected move:    +5% (bounce expected)

Position calculation:
  Max loss: $50
  Position size = $50 / ($2/$62) = $50 / 0.032 = $1,562 ✗ TOO LARGE

  Cap at 25%: $250 (25% of capital)

Expected profit: $250 × 5% = +$12.50
Risk/Reward: $12.50 / $50 = 0.25:1 ❌ STILL MEH
```

**BUT WAIT:** At $62, if bounce is successful, you're only $2 away from massive upside. And RSI 12 is EXTREME capitulation. This might be THE bottom.

**Decision: Take position at $62, but scaled:**

```
Entry 1 @ $62: Position size $200 (20% of capital)
  Expected profit: $200 × 5% = +$10
  Max loss if support breaks: $2 × (200/$62) = $6.45
  Risk/Reward: $10 / $6.45 = 1.55:1 ✓ ACCEPTABLE

If support holds, add Entry 2 at $59 (5% drop):
  New runway: $1
  Technical confidence even higher (extreme capitulation)
  Entry 2: Position size $100 (10% of capital, smaller because runway shrunk)

Total: $300 deployed if both entries happen
```

---

## The Algorithm: Pain vs Reward

### Position Sizing Formula

```
position_size = acceptable_risk / (distance_to_support / current_price)

Where:
- acceptable_risk = 5% of capital = $50 (hard limit)
- distance_to_support = current - support
- current_price = entry price

Example at $62 with $60 support:
  position_size = $50 / (($62 - $60) / $62)
  position_size = $50 / 0.0323
  position_size = $1,548

  Cap at 25% of capital: $250 max
  Final: min($1,548, $250) = $250
```

### Risk/Reward Validation

```
Before trading, check:
  Expected profit ≥ 2× acceptable loss? (2:1 minimum)
  OR
  Technical confidence VERY high (RSI < 15, multiple signals)?

If neither, SKIP the trade.
Position size alone doesn't make a bad trade good.
```

---

## DCA with Dynamic Sizing

Once Entry 1 is taken, scale Entry 2, 3, 4 based on original Entry 1 size:

```
Entry 1 @ $62: $200 position (100%)
Entry 2 @ $59: $160 position (80% of Entry 1)
Entry 3 @ $56: $120 position (60% of Entry 1)
Entry 4 @ $53: $80 position (40% of Entry 1)

Total possible deployment: $560 (56% of capital)
Keeps 44% reserve for mistakes
```

**Why scale down?**
- Deeper you go, lower your confidence should be
- Runway to support keeps shrinking
- Better to have smaller positions in uncertainty zone
- Preserves capital if thesis completely wrong

---

## Decision Tree: When to Trade

```
1. Identify support/resistance levels (chart analysis)
   ↓
2. Wait for technical setup (RSI < 30 OR > 70, MACD cross, Bollinger extreme)
   ↓
3. Calculate position size: acceptable_risk / (distance_to_support / price)
   ↓
4. Check risk/reward:
   - Is expected_profit ≥ 2× max_loss? ✓ TRADE
   - OR Is technical extreme (RSI < 15)? ✓ TRADE SMALL
   - Else? ✗ SKIP
   ↓
5. If approved, enter with DCA plan:
   - Entry 1: Full calculated size (100%)
   - Entry 2: 80% (if another dip)
   - Entry 3: 60% (if dip continues)
   - Entry 4: 40% (extreme only)
```

---

## SOL Example: Full Scenario

### Scenario A: Bounces at $62

```
Entry 1 @ $62:    Position $200, technical confidence HIGH
Price recovers to $68 (+9.7%)
Profit: $200 × 9.7% = +$19.40
No need for Entry 2

Exit: Price hits $70 (your resistance target)
Final profit: $200 × 12.9% = +$25.80

Monthly if 1-2 bounces like this: +$50-75 per bounce
```

### Scenario B: Breaks Support - You're Protected

```
Entry 1 @ $62:    Position $200
Entry 2 @ $59:    Position $160 (if you're confident)

Thesis breaks, support falls to $55
Max loss on Entry 1: $200 × ($62-$55)/$62 = $22.58
Max loss on Entry 2: $160 × ($59-$55)/$59 = $10.81
Total loss: ~$33.39

Still within 5% acceptable risk!
($33.39 / $1,000 = 3.3%, under 5% limit)
```

---

## Why This Works Better Than Fixed Sizing

### Fixed 25% Per Entry (Old Approach)
```
Entry 1 @ $82: 25% = $250
Entry 2 @ $75: 25% = $250
Entry 3 @ $68: 25% = $250
Entry 4 @ $60: 25% = $250
Total: 100% of capital, all @ 10x = 10x leverage on entire position

If thesis breaks at $60:
  Total loss: ~$1,000 (entire position)
  Reason: Each entry was sized identically despite runway shrinking
```

### Dynamic Sizing (New Approach)
```
Entry 1 @ $82: Risk/Reward bad, skip
Entry 2 @ $75: Risk/Reward still bad, skip
Entry 3 @ $62: Position $200 (20%)
Entry 4 @ $59: Position $160 (16%)
Entry 5 @ $56: Position $120 (12%)
Total: 48% deployed, positions SMALLER as runway shrinks

If thesis breaks:
  Total loss: ~$35 (3.5% of capital)
  Reason: Smaller positions where support distance is tightest
```

**Difference: You protect capital by sizing smaller when most uncertain.**

---

## Implementation in Code

```rust
// Calculate entry size
let support_resistance = SupportResistance::new(60.0, 85.0, 82.0);
let technical = TechnicalSetup {
    rsi: 28.0,
    rsi_oversold: 30.0,
    macd_above_signal: true,
    price_vs_bollinger: -0.8,
    // ...
};

let position = DynamicSizer::calculate_position_size(
    1000.0,           // capital
    &support_resistance,
    &technical,
    0.05              // 5% max risk per trade
);

if position.is_viable && position.risk_reward_ratio >= 2.0 {
    // Execute trade with position.position_size_dollars
    // Scale DCA entries by 80%, 60%, 40%
}
```

---

## Testing Your Theory

This automated system should validate:

1. **Support/Resistance Identification**
   - Can you consistently identify levels that actually hold?
   - Do oversold bounces at support outperform random entries?

2. **Technical Entry Timing**
   - Does RSI < 30 + MACD + Bollinger extreme = better entries?
   - Can you catch reversals before they run 5%+?

3. **Dynamic Position Sizing**
   - Does sizing smaller near support = smaller losses?
   - Do you capture enough upside to justify the smaller positions?

4. **Overall Edge**
   - Monthly returns with dynamic sizing vs fixed sizing
   - Win rate improvement
   - Max drawdown reduction

---

## Expected Results (If Theory Holds)

```
Monthly trades: 8-12
Average setup: Support bounce with RSI < 30

Per trade:
  Position size: 10-20% (varies with support distance)
  Expected move: 2-4%
  Average profit: $30-50 per win
  Acceptable loss: $30-50 if support breaks

Monthly:
  Wins: 6-8 trades
  Losses: 2-4 trades
  Total profit: 6 × $40 - 3 × $35 = $240 - $105 = +$135
  Return: +13.5% on $1,000

This assumes:
- Support/resistance correctly identified
- Technical signals meaningful
- No multiple entry cascades (avoid over-leverage)
```

---

## The Core Principle

**You don't need to know the exact bottom.** You just need to:

1. Identify a reasonable support level (chart analysis)
2. Wait for extreme technical setup (multiple indicators)
3. Size position small enough that if you're wrong, loss is acceptable
4. Scale entries down as you go deeper (avoid compounding mistakes)

This is how professional traders manage risk while building positions.

**It automates what you've been doing manually.** Now let's test if it works.
