use anyhow::Result;
use serde::{Deserialize, Serialize};
use crate::data::OrderBook;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderFlowSignal {
    pub bid_volume: f64,
    pub ask_volume: f64,
    pub imbalance_ratio: f64,
    pub direction: String,
    pub confidence: f64,
}

pub fn detect_order_flow(orderbook: &OrderBook) -> Result<OrderFlowSignal> {
    let bid_volume: f64 = orderbook.bids.iter().map(|(_, vol)| vol).sum();
    let ask_volume: f64 = orderbook.asks.iter().map(|(_, vol)| vol).sum();
    
    let imbalance_ratio = if ask_volume > 0.0 {
        bid_volume / ask_volume
    } else {
        1.0
    };

    let direction = if imbalance_ratio > 1.5 {
        "LONG".to_string()
    } else if imbalance_ratio < 0.67 {
        "SHORT".to_string()
    } else {
        "NEUTRAL".to_string()
    };

    // Mirror LONG thresholds for SHORT: invert the ratio.
    // Ratio 0.33 = asks are 3× bids  (symmetric with ratio 3.0)
    // Ratio 0.50 = asks are 2× bids  (symmetric with ratio 2.0)
    // Ratio 0.67 = asks are 1.5× bids (symmetric with ratio 1.5)
    let confidence = match imbalance_ratio {
        r if r > 3.0  => 0.95,  // bids 3× asks → strong LONG
        r if r > 2.0  => 0.85,
        r if r > 1.5  => 0.70,
        r if r < 0.33 => 0.95,  // asks 3× bids → strong SHORT (was missing!)
        r if r < 0.50 => 0.85,
        r if r < 0.67 => 0.70,
        _ => 0.50,               // balanced or barely imbalanced
    };

    Ok(OrderFlowSignal {
        bid_volume,
        ask_volume,
        imbalance_ratio,
        direction,
        confidence,
    })
}

// ═══════════════════════════════════════════════════════════════════════════════
//  UNIT TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::OrderBook;

    // ── Helpers ──────────────────────────────────────────────────────────────

    fn book(bid_vol: f64, ask_vol: f64) -> OrderBook {
        // Single level each side — enough to drive imbalance_ratio
        OrderBook {
            symbol:    "TEST".to_string(),
            timestamp: 0,
            bids: if bid_vol > 0.0 { vec![(100.0, bid_vol)] } else { vec![] },
            asks: if ask_vol > 0.0 { vec![(100.1, ask_vol)] } else { vec![] },
        }
    }

    fn multi_level_book(bids: &[(f64, f64)], asks: &[(f64, f64)]) -> OrderBook {
        OrderBook {
            symbol:    "TEST".to_string(),
            timestamp: 0,
            bids: bids.to_vec(),
            asks: asks.to_vec(),
        }
    }

    // ── Direction ────────────────────────────────────────────────────────────

    #[test]
    fn direction_long_when_bids_dominate() {
        let sig = detect_order_flow(&book(300.0, 100.0)).unwrap();
        assert_eq!(sig.direction, "LONG", "3:1 bid:ask should be LONG");
    }

    #[test]
    fn direction_short_when_asks_dominate() {
        let sig = detect_order_flow(&book(100.0, 300.0)).unwrap();
        assert_eq!(sig.direction, "SHORT", "1:3 bid:ask should be SHORT");
    }

    #[test]
    fn direction_neutral_when_balanced() {
        let sig = detect_order_flow(&book(100.0, 100.0)).unwrap();
        assert_eq!(sig.direction, "NEUTRAL", "1:1 should be NEUTRAL");
    }

    #[test]
    fn direction_neutral_boundary_just_above_threshold() {
        // imbalance = 1.5 exactly hits the > 1.5 boundary → NOT long (needs > 1.5)
        let sig = detect_order_flow(&book(150.0, 100.0)).unwrap();
        // ratio = 1.5, which is NOT > 1.5, so should be NEUTRAL
        assert_eq!(sig.direction, "NEUTRAL",
            "ratio exactly 1.5 is the threshold boundary — should be NEUTRAL");
    }

    #[test]
    fn direction_long_just_above_threshold() {
        let sig = detect_order_flow(&book(151.0, 100.0)).unwrap();
        assert_eq!(sig.direction, "LONG", "151:100 = 1.51 ratio should be LONG");
    }

    // ── Confidence LONG ───────────────────────────────────────────────────────

    #[test]
    fn confidence_long_tier_1_above_3x() {
        let sig = detect_order_flow(&book(400.0, 100.0)).unwrap();
        assert_eq!(sig.confidence, 0.95, "4:1 bid:ask should yield 0.95 confidence");
    }

    #[test]
    fn confidence_long_tier_2_above_2x() {
        let sig = detect_order_flow(&book(250.0, 100.0)).unwrap();
        assert_eq!(sig.confidence, 0.85, "2.5:1 bid:ask should yield 0.85 confidence");
    }

    #[test]
    fn confidence_long_tier_3_above_1_5x() {
        let sig = detect_order_flow(&book(180.0, 100.0)).unwrap();
        assert_eq!(sig.confidence, 0.70, "1.8:1 bid:ask should yield 0.70 confidence");
    }

    // ── Confidence SHORT — REGRESSION (previously all returned 0.50) ──────────

    #[test]
    fn confidence_short_tier_1_asks_3x_bids_regression() {
        // Pre-fix: imbalance_ratio = 0.25 hit the `_ => 0.50` arm.
        // Post-fix: should return 0.95 (symmetric with LONG tier 1).
        let sig = detect_order_flow(&book(100.0, 400.0)).unwrap();
        assert_eq!(sig.direction, "SHORT");
        assert_eq!(sig.confidence, 0.95,
            "REGRESSION: 1:4 ask:bid (ratio=0.25) should be 0.95, was 0.50 before fix");
    }

    #[test]
    fn confidence_short_tier_2_asks_2x_bids_regression() {
        let sig = detect_order_flow(&book(100.0, 250.0)).unwrap();
        assert_eq!(sig.direction, "SHORT");
        assert_eq!(sig.confidence, 0.85,
            "REGRESSION: 1:2.5 ask:bid (ratio≈0.40) should be 0.85, was 0.50 before fix");
    }

    #[test]
    fn confidence_short_tier_3_asks_1_5x_bids_regression() {
        let sig = detect_order_flow(&book(100.0, 180.0)).unwrap();
        assert_eq!(sig.direction, "SHORT");
        assert_eq!(sig.confidence, 0.70,
            "REGRESSION: 1:1.8 ask:bid (ratio≈0.56) should be 0.70, was 0.50 before fix");
    }

    #[test]
    fn confidence_short_symmetry_with_long() {
        // Long at 4:1 and Short at 1:4 should have identical confidence
        let long_sig  = detect_order_flow(&book(400.0, 100.0)).unwrap();
        let short_sig = detect_order_flow(&book(100.0, 400.0)).unwrap();
        assert_eq!(long_sig.confidence, short_sig.confidence,
            "Long (4:1) and Short (1:4) must have symmetric confidence");
    }

    #[test]
    fn confidence_neutral_returns_050() {
        let sig = detect_order_flow(&book(120.0, 100.0)).unwrap();
        // ratio = 1.2, inside the NEUTRAL zone
        assert_eq!(sig.direction, "NEUTRAL");
        assert_eq!(sig.confidence, 0.50, "neutral zone should always be 0.50");
    }

    // ── Edge cases ────────────────────────────────────────────────────────────

    #[test]
    fn empty_asks_gives_neutral_imbalance() {
        let b = multi_level_book(&[(100.0, 50.0)], &[]);
        let sig = detect_order_flow(&b).unwrap();
        // ask_volume = 0 → imbalance fallback to 1.0 → NEUTRAL
        assert_eq!(sig.imbalance_ratio, 1.0, "zero asks should produce 1.0 ratio");
        assert_eq!(sig.direction, "NEUTRAL");
    }

    #[test]
    fn empty_book_gives_neutral() {
        let b = multi_level_book(&[], &[]);
        let sig = detect_order_flow(&b).unwrap();
        assert_eq!(sig.direction, "NEUTRAL", "empty book should be NEUTRAL");
        assert_eq!(sig.confidence, 0.50, "empty book should have 0.50 confidence");
    }

    #[test]
    fn multi_level_volumes_summed_correctly() {
        // 3 bid levels totalling 300, 2 ask levels totalling 100 → ratio = 3.0
        let b = multi_level_book(
            &[(100.5, 100.0), (100.0, 150.0), (99.5, 50.0)],
            &[(100.6,  60.0), (101.0,  40.0)],
        );
        let sig = detect_order_flow(&b).unwrap();
        assert_eq!(sig.bid_volume, 300.0);
        assert_eq!(sig.ask_volume, 100.0);
        // ratio = 3.0 exactly → NOT > 3.0 → tier 2 (> 2.0)
        assert_eq!(sig.confidence, 0.85);
    }

    #[test]
    fn imbalance_ratio_computed_correctly() {
        let sig = detect_order_flow(&book(200.0, 80.0)).unwrap();
        let expected = 200.0 / 80.0;
        assert!((sig.imbalance_ratio - expected).abs() < 1e-10,
            "imbalance_ratio should be bid/ask, got {}", sig.imbalance_ratio);
    }
}
