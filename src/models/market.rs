/// Market data structures for trading
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Market data snapshot
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MarketData {
    pub symbol: String,
    pub bid: f64,
    pub ask: f64,
    pub last_price: f64,
    pub volume_24h: f64,
    pub volatility: f64,
    pub momentum: f64,
    pub timestamp: i64,
}

impl MarketData {
    pub fn mid_price(&self) -> f64 {
        (self.bid + self.ask) / 2.0
    }

    pub fn spread(&self) -> f64 {
        self.ask - self.bid
    }

    pub fn spread_percentage(&self) -> f64 {
        (self.spread() / self.mid_price()) * 100.0
    }
}

/// Order book with bids and asks
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrderBook {
    pub symbol: String,
    pub bids: Vec<(f64, f64)>,  // (price, size)
    pub asks: Vec<(f64, f64)>,
    pub timestamp: i64,
}

impl OrderBook {
    pub fn best_bid(&self) -> Option<f64> {
        self.bids.first().map(|(price, _)| *price)
    }

    pub fn best_ask(&self) -> Option<f64> {
        self.asks.first().map(|(price, _)| *price)
    }

    pub fn mid_price(&self) -> Option<f64> {
        match (self.best_bid(), self.best_ask()) {
            (Some(bid), Some(ask)) => Some((bid + ask) / 2.0),
            _ => None,
        }
    }

    pub fn bid_volume(&self, levels: usize) -> f64 {
        self.bids
            .iter()
            .take(levels)
            .map(|(_, size)| size)
            .sum()
    }

    pub fn ask_volume(&self, levels: usize) -> f64 {
        self.asks
            .iter()
            .take(levels)
            .map(|(_, size)| size)
            .sum()
    }
}

/// Limit order for placing on exchanges
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LimitOrder {
    pub symbol: String,
    pub side: OrderSide,
    pub price: f64,
    pub size: f64,
    pub leverage: f64,
    pub post_only: bool,
}

/// Market order for immediate execution
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MarketOrder {
    pub symbol: String,
    pub side: OrderSide,
    pub size: f64,
    pub leverage: f64,
}

/// Generic order
#[derive(Clone, Debug)]
pub enum Order {
    Limit(LimitOrder),
    Market(MarketOrder),
}

/// Order side
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderSide {
    Buy,
    Sell,
}

impl OrderSide {
    pub fn is_buy(&self) -> bool {
        matches!(self, OrderSide::Buy)
    }

    pub fn is_sell(&self) -> bool {
        matches!(self, OrderSide::Sell)
    }

    pub fn opposite(&self) -> Self {
        match self {
            OrderSide::Buy => OrderSide::Sell,
            OrderSide::Sell => OrderSide::Buy,
        }
    }
}

/// Trading position on an exchange
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Position {
    pub symbol: String,
    pub side: OrderSide,
    pub size: f64,
    pub entry_price: f64,
    pub mark_price: f64,
    pub liquidation_price: f64,
    pub unrealized_pnl: f64,
    pub leverage: f64,
    pub timestamp: i64,
}

impl Position {
    pub fn margin_ratio(&self) -> f64 {
        if self.mark_price > 0.0 {
            (self.mark_price * self.size) / (self.mark_price * self.size * self.leverage)
        } else {
            0.0
        }
    }

    pub fn time_to_liquidation(&self) -> Option<u64> {
        let current_distance = if self.side.is_buy() {
            self.mark_price - self.liquidation_price
        } else {
            self.liquidation_price - self.mark_price
        };

        if current_distance > 0.0 {
            Some((current_distance / self.mark_price * 1000.0) as u64)
        } else {
            None
        }
    }
}

/// Trade fill information
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Fill {
    pub order_id: String,
    pub symbol: String,
    pub side: OrderSide,
    pub price: f64,
    pub size: f64,
    pub timestamp: i64,
    pub fee: f64,
    pub fee_asset: String,
}

impl Fill {
    pub fn cost(&self) -> f64 {
        self.price * self.size
    }

    pub fn total_cost_with_fee(&self) -> f64 {
        self.cost() + self.fee
    }
}

/// Account information from exchange
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountInfo {
    pub account_id: String,
    pub total_equity: f64,
    pub available_balance: f64,
    pub used_margin: f64,
    pub free_margin: f64,
    pub margin_ratio: f64,
    pub cross_margin: f64,
    pub timestamp: i64,
}

impl AccountInfo {
    pub fn can_trade(&self) -> bool {
        self.margin_ratio > 0.0 && self.margin_ratio > 1.5
    }

    pub fn liquidation_risk(&self) -> f64 {
        if self.cross_margin > 0.0 {
            self.total_equity / self.cross_margin
        } else {
            1.0
        }
    }
}

/// Trading signal from analysis
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Signal {
    pub symbol: String,
    pub signal_type: SignalType,
    pub strength: f64,  // -1.0 to 1.0
    pub confidence: f64,  // 0.0 to 1.0
    pub timestamp: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SignalType {
    Technical {
        pattern: String,
        timeframe: String,
    },
    Sentiment {
        source: String,
        score: f64,
    },
    OnChain {
        metric: String,
        value: f64,
    },
    ML {
        model: String,
        features: HashMap<String, f64>,
    },
}

/// Market analysis result
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MarketAnalysis {
    pub timestamp: i64,
    pub volatility_percentile: f64,
    pub bid_ask_spread: f64,
    pub volume_24h: f64,
    pub price_momentum: f64,
    pub slippage: f64,
    pub trading_regime: TradingRegime,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum TradingRegime {
    Trending,
    Ranging,
    HighVolatility,
    LowVolatility,
    Unknown,
}

/// OHLCV candle data
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OHLCV {
    pub timestamp: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub interval: String,  // "1m", "5m", "1h", "1d"
}

impl OHLCV {
    pub fn pct_change(&self) -> f64 {
        ((self.close - self.open) / self.open) * 100.0
    }

    pub fn range(&self) -> f64 {
        self.high - self.low
    }
}

/// Execution result from order
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub order_id: String,
    pub symbol: String,
    pub status: ExecutionStatus,
    pub average_price: f64,
    pub filled_size: f64,
    pub remaining_size: f64,
    pub timestamp: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ExecutionStatus {
    Pending,
    PartiallyFilled,
    Filled,
    Rejected,
    Cancelled,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_market_data_mid_price() {
        let data = MarketData {
            symbol: "SOLUSDT".to_string(),
            bid: 100.0,
            ask: 102.0,
            last_price: 101.0,
            volume_24h: 1000000.0,
            volatility: 0.05,
            momentum: 0.1,
            timestamp: 0,
        };

        assert_eq!(data.mid_price(), 101.0);
        assert_eq!(data.spread(), 2.0);
        assert!((data.spread_percentage() - 1.98).abs() < 0.01);
    }

    #[test]
    fn test_order_book_best_prices() {
        let book = OrderBook {
            symbol: "SOLUSDT".to_string(),
            bids: vec![(100.0, 10.0), (99.0, 20.0)],
            asks: vec![(102.0, 10.0), (103.0, 20.0)],
            timestamp: 0,
        };

        assert_eq!(book.best_bid(), Some(100.0));
        assert_eq!(book.best_ask(), Some(102.0));
        assert_eq!(book.mid_price(), Some(101.0));
    }

    #[test]
    fn test_order_side_opposite() {
        assert_eq!(OrderSide::Buy.opposite(), OrderSide::Sell);
        assert_eq!(OrderSide::Sell.opposite(), OrderSide::Buy);
    }

    #[test]
    fn test_position_pnl() {
        let position = Position {
            symbol: "SOLUSDT".to_string(),
            side: OrderSide::Buy,
            size: 10.0,
            entry_price: 100.0,
            mark_price: 105.0,
            liquidation_price: 50.0,
            unrealized_pnl: 50.0,
            leverage: 2.0,
            timestamp: 0,
        };

        assert!(position.unrealized_pnl > 0.0);
        assert!(position.time_to_liquidation().is_some());
    }

    #[test]
    fn test_fill_cost() {
        let fill = Fill {
            order_id: "1".to_string(),
            symbol: "SOLUSDT".to_string(),
            side: OrderSide::Buy,
            price: 100.0,
            size: 10.0,
            timestamp: 0,
            fee: 2.0,
            fee_asset: "USDC".to_string(),
        };

        assert_eq!(fill.cost(), 1000.0);
        assert_eq!(fill.total_cost_with_fee(), 1002.0);
    }

    #[test]
    fn test_ohlcv_pct_change() {
        let candle = OHLCV {
            timestamp: 0,
            open: 100.0,
            high: 110.0,
            low: 95.0,
            close: 105.0,
            volume: 1000000.0,
            interval: "1h".to_string(),
        };

        assert!((candle.pct_change() - 5.0).abs() < 0.01);
        assert_eq!(candle.range(), 15.0);
    }
}
