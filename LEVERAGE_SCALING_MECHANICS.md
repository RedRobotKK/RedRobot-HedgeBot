# ⚙️ Leverage Scaling Mechanics: How It Actually Works

## The Key Insight: Each Entry = Separate Order = Separate Leverage

On DEXs like **Hyperliquid/Drift**, leverage is set **per-order**, NOT per-position. This means:

```
Entry 1: 1.0 SOL @ 10x leverage → Creates order 1 with 10x
Entry 2: 1.1 SOL @ 8x leverage  → Creates order 2 with 8x (separate)
Entry 3: 1.5 SOL @ 5x leverage  → Creates order 3 with 5x (separate)

Result: ONE POSITION with 3 separate orders at different leverage levels
Weighted average leverage: (1.0×10 + 1.1×8 + 1.5×5) / 3.6 = 7.5x effective
```

---

## How Leverage Per-Entry Works (Hyperliquid)

### Step-by-Step Example

```
INITIAL ACCOUNT:
  Balance: $1,000 USDC
  Leverage setting: 10x (default)

ENTRY 1: Long SOL at 10x leverage
  Command: BUY 10 SOL with 10x leverage
  Capital used: 10 SOL × $100/SOL = $1,000 notional
  Margin used: $1,000 / 10x = $100 (10% of balance)
  Remaining balance: $900

ENTRY 2: ADD with 8x leverage (LOWER leverage)
  Command: BUY +8 SOL with 8x leverage
  Capital used: 8 SOL × $98/SOL = $784 notional
  Margin used: $784 / 8x = $98 (9.8% of remaining)
  Remaining balance: $802

ENTRY 3: ADD with 5x leverage (EVEN LOWER)
  Command: BUY +5 SOL with 5x leverage
  Capital used: 5 SOL × $95/SOL = $475 notional
  Margin used: $475 / 5x = $95 (9.5% of remaining)
  Remaining balance: $707
```

---

## Position Aggregation: How Hyperliquid Handles It

### The Math Behind It

When you place multiple orders at different leverage on **Hyperliquid**, the system:

1. **Tracks each order separately** with its leverage
2. **Aggregates quantity** into one position
3. **Calculates effective leverage** as weighted average

```
Order 1: 10 SOL @ 10x = $100 margin
Order 2: 8 SOL @ 8x   = $80 margin
Order 3: 5 SOL @ 5x   = $50 margin

Total position: 23 SOL
Total margin: $230
Effective leverage: ($100 + $80 + $50) / $1000 capital × leverage_factor
                  = $230 / $1000 × ~4.35 = 7.7x effective
```

---

## Why Scale DOWN Leverage on Each Entry?

### The Risk Management Logic

**Entry 1 (10x):** High conviction, first signal hit, full confidence
- You're betting the most per dollar of margin
- If price moves 10%, you gain/lose 100% of Entry 1 margin

**Entry 2 (8x):** Medium conviction, averaging down, support held
- You're reducing risk per dollar of margin
- If price moves 10%, you gain/lose 80% of Entry 2 margin (vs 100% Entry 1)

**Entry 3 (5x):** Lower conviction at this point, deeper dip, last chance
- You're being very conservative with new capital
- If price moves 10%, you gain/lose 50% of Entry 3 margin (safest)

### The Actual Protection

```
Scenario: Price drops 10%

Entry 1 (10x):
  Position size: 10 SOL @ $100 = $1,000 notional
  Loss: 10% = -$100
  Margin impact: -$100 / 10x = -$10 loss on margin account

Entry 2 (8x):
  Position size: 8 SOL @ $98 = $784 notional
  Loss: 10% = -$78.40
  Margin impact: -$78.40 / 8x = -$9.80 loss on margin account

Entry 3 (5x):
  Position size: 5 SOL @ $95 = $475 notional
  Loss: 10% = -$47.50
  Margin impact: -$47.50 / 5x = -$9.50 loss on margin account

Total margin loss: $10 + $9.80 + $9.50 = $29.30
Total position loss: $100 + $78.40 + $47.50 = $225.90

BUT: Position is 23 SOL. If you'd entered all 23 @ 10x:
  Loss would be: $225.90 across 23 SOL = 10% × $2,300 = $230 loss (WORSE)

By scaling down leverage: You limited max loss to $29.30 vs $30+
```

---

## Three Approaches to Leverage Management

### Approach 1: FIXED Leverage (Simplest)
```
Entry 1: 10x
Entry 2: 10x
Entry 3: 10x
Entry 4: 10x

Risk: If 4 entries and price moves against you, total liquidation risk compounds
Weighted avg: 10x (max risk)
Best for: Confident entry, no deeper dips expected
```

### Approach 2: SCALING DOWN Leverage (Recommended) ✅
```
Entry 1: 10x (high conviction)
Entry 2: 8x  (medium conviction, support tested)
Entry 3: 5x  (lower conviction, deep dip)
Entry 4: 3x  (very low conviction, extreme dip)

Risk: Each entry has lower leverage, total liquidation risk stays manageable
Weighted avg: ~6.5x (controlled)
Best for: Averaging down strategy, wanting to add at lower prices
```

### Approach 3: SCALING UP Leverage (Aggressive)
```
Entry 1: 5x  (cautious entry)
Entry 2: 8x  (confidence building)
Entry 3: 10x (strong conviction, price bouncing)
Entry 4: 15x (maximum conviction)

Risk: Leverage INCREASES as position underwater, liquidation risk EXPLODES
Weighted avg: 9.5x (risky)
Best for: Scaling INTO winners (price going UP), not averaging down
```

---

## CRITICAL: When to Use Which Approach

### Use SCALING DOWN (10x → 8x → 5x → 3x) When:
- ✅ Averaging DOWN (price falling, adding to loser)
- ✅ Using DCA strategy
- ✅ Adding on dips (uncertainty increases)
- ✅ Want to LIMIT liquidation risk

**This protects you because:**
Each additional entry has less leverage → lower liquidation risk → can afford more entries without blowing up

### Use FIXED Leverage (10x → 10x → 10x) When:
- ✅ Very high confidence (8-9 signals)
- ✅ Price is falling slightly but support holding
- ✅ Plan to add only 1-2 times max
- ✅ Have capital for worst case

### NEVER Use SCALING UP (5x → 8x → 10x → 15x) When:
- ❌ Averaging DOWN (adding to losers)
- ❌ You're uncertain (that's when leverage should lower)
- ❌ You want safety (compounding risk is dangerous)

**This is dangerous because:**
You're doubling down with HIGHER leverage when position is worst → liquidation trap

---

## Real Math: Three Scenarios

### Scenario 1: DCA with Scaling Down Leverage (SAFEST)

```
Starting capital: $1,000
Target position: Build to 25 SOL, avg cost $95

Entry 1: 10 SOL @ $100, 10x leverage
  Margin: $100 / 10x = $10 used
  Position value: $1,000 notional

Entry 2: 8 SOL @ $96, 8x leverage
  Margin: $768 / 8x = $96 used
  Position value: $768 notional

Entry 3: 7 SOL @ $92, 5x leverage
  Margin: $644 / 5x = $128.80 used
  Position value: $644 notional

Total:
  Quantity: 25 SOL
  Average entry: $95.52
  Total margin: $10 + $96 + $128.80 = $234.80
  Effective leverage: $234.80 / $1,000 = 4.26x equiv

Liquidation risk: MODERATE
- Weighted avg leverage: ~7.5x effective
- But each entry has its own separate stop
- Can close Entry 3 at 5x, then Entry 2 at 8x, then Entry 1 at 10x
- Gradual unwinding, not catastrophic
```

### Scenario 2: Fixed High Leverage (RISKY)

```
Same entries, but all 10x leverage:

Entry 1: 10 SOL @ $100, 10x leverage
  Margin: $100 used

Entry 2: 8 SOL @ $96, 10x leverage
  Margin: $77 used

Entry 3: 7 SOL @ $92, 10x leverage
  Margin: $64.40 used

Total:
  Quantity: 25 SOL
  Total margin: $241.40
  Effective leverage: 10x FLAT

Liquidation risk: HIGH
- All entries at 10x = maximum leverage throughout
- Position gets liquidated faster if price drops
- No scaling of risk = riskier
```

### Scenario 3: Scaling UP Leverage (DANGEROUS FOR AVERAGING DOWN)

```
Same entries, but leverage INCREASES:

Entry 1: 10 SOL @ $100, 5x leverage
  Margin: $200 used

Entry 2: 8 SOL @ $96, 10x leverage
  Margin: $77 used

Entry 3: 7 SOL @ $92, 15x leverage
  Margin: $43 used

Total:
  Quantity: 25 SOL
  Total margin: $320
  Effective leverage: 12.8x effective

Liquidation risk: CRITICAL
- You're at lowest leverage when MOST confident (Entry 1)
- You're at HIGHEST leverage when LEAST confident (Entry 3)
- If Entry 3 drops 6%, gets liquidated at 15x
- Then Entry 2 drops harder, gets liquidated at 10x
- THEN Entry 1 gets left holding bag at 5x
- Cascade liquidation = total wipeout

⚠️ NEVER DO THIS FOR AVERAGING DOWN
(OK for scaling INTO WINNERS, different strategy)
```

---

## How Hyperliquid Actually Processes This

### Order-By-Order Execution

```
Step 1: Place Order 1
  Leverage: 10x
  Order type: Market (instant fill)
  Result: 10 SOL position @ 10x leverage

Step 2: Place Order 2 (same symbol, different leverage)
  Leverage: 8x (CHANGE leverage first, then order)
  Order type: Market (instant fill)
  Result: Position now 18 SOL, but tracked as:
    - 10 SOL @ 10x (Order 1)
    - 8 SOL @ 8x (Order 2)

Step 3: Place Order 3 (change leverage again)
  Leverage: 5x (CHANGE leverage to 5x)
  Order type: Market (instant fill)
  Result: Position now 25 SOL (aggregated), but tracked as:
    - 10 SOL @ 10x
    - 8 SOL @ 8x
    - 7 SOL @ 5x
```

### Hyperliquid UI Controls

```
When in a position with multiple entries at different leverage:

Current position: 25 SOL long
  Entry details:
    [10.0 SOL]  Leverage: 10x   Entry price: $100.00
    [8.0 SOL]   Leverage: 8x    Entry price: $96.00
    [7.0 SOL]   Leverage: 5x    Entry price: $92.00

  Effective leverage indicator: 7.5x (weighted avg)
  Liquidation price: $87.50 (based on 7.5x and margin)

  Close options:
    ✓ Close all (closes all 3 orders together)
    ✓ Reduce position (closes proportionally)
    ✓ Set stop loss (applies to whole position)
    ✓ Set take profit (applies to whole position)
```

---

## Key Insight: Individual Entry Leverage vs Position Leverage

### Individual Entry (Order) Level:
```
Entry 1 @ 10x: This specific $1,000 notional is on 10x margin
Entry 2 @ 8x:  This specific $768 notional is on 8x margin
Entry 3 @ 5x:  This specific $644 notional is on 5x margin

If Entry 3 (5x) gets liquidated: Only that $644 notional closes
Remaining: Entry 1 ($1,000 @ 10x) + Entry 2 ($768 @ 8x) still open
```

### Position Level (Aggregate):
```
Total position: 25 SOL
Liquidation price: $87.50 (weighted average calculation)
This is where the ENTIRE 25 SOL position gets liquidated if price hits $87.50
```

---

## Recommendation: Leverage Scaling for DCA

### Safe Pyramid (Our System)

```
Entry 1 (Confluence 0.75): 10x leverage
  → High confidence, first signal, use full leverage

Entry 2 (Confluence 0.75): 8x leverage
  → Medium confidence, averaged down, reduce leverage

Entry 3 (Confluence 0.80): 5x leverage
  → Lower confidence on deeper dip, very conservative

Entry 4 (Confluence 0.85): 3x leverage
  → Extreme dip, maximum requirements, minimal leverage

Why this works:
- Entry 1 has most leverage (most confident)
- Each additional entry = more capital at risk
- But leverage DECREASES = compensates for additional risk
- Total liquidation risk stays STABLE across all 4 entries
```

### Capital Requirement Check

```
Entry 1 @ 10x: $100 margin (10% of $1,000)
Entry 2 @ 8x:  $96 margin (9.6% of $1,000)
Entry 3 @ 5x:  $127 margin (12.7% of $1,000)
Entry 4 @ 3x:  $167 margin (16.7% of $1,000)

Total margin needed: $490 (49% of capital)
Remaining capital: $510 (safety buffer for adverse moves)
```

---

## Summary: Leverage Scaling is Key to Safe DCA

✅ **DO Scale DOWN on each entry**
- Reduces risk as you add more capital
- Prevents liquidation cascade
- Allows more flexibility (can close entries individually)

❌ **DON'T use fixed high leverage on all entries**
- Compounds risk with each addition
- All entries liquidate together

❌ **NEVER scale UP on entries when averaging down**
- Suicidal for DCA strategy
- Max risk when least confident
- Cascade liquidation trap

**Your system now tracks this automatically:**
```
Entry 1: leverage = 10.0
Entry 2: leverage = 8.0  ← Automatically calculated and reduced
Entry 3: leverage = 5.0  ← Further reduced
Entry 4: leverage = 3.0  ← Even more conservative
```

This is the **professional approach** that separates risk-managed traders from reckless ones.
