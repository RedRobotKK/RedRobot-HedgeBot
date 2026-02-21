/// Account data structures for multi-protocol trading
use crate::utils::Result;
use serde::{Deserialize, Serialize};
use std::default::Default;

/// Protocol enumeration
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Protocol {
    Drift,
    Hyperliquid,
    Phantom,
}

impl Protocol {
    pub fn as_str(&self) -> &str {
        match self {
            Protocol::Drift => "drift",
            Protocol::Hyperliquid => "hyperliquid",
            Protocol::Phantom => "phantom",
        }
    }
}

/// Trading account purpose/strategy
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AccountPurpose {
    /// High-frequency scalping: 100x leverage, 2% stops, 4% targets
    Scalp,
    /// Swing trading: 20x leverage, medium-term holds
    Swing,
    /// Position trading: 10x leverage, longer-term holds
    Position,
    /// Defensive hedging: 5x leverage, portfolio protection
    Hedge,
    /// Emergency reserve: 0x leverage, no trading
    Reserve,
}

impl AccountPurpose {
    pub fn max_leverage(&self) -> f64 {
        match self {
            AccountPurpose::Scalp => 100.0,
            AccountPurpose::Swing => 20.0,
            AccountPurpose::Position => 10.0,
            AccountPurpose::Hedge => 5.0,
            AccountPurpose::Reserve => 0.0,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            AccountPurpose::Scalp => "scalp",
            AccountPurpose::Swing => "swing",
            AccountPurpose::Position => "position",
            AccountPurpose::Hedge => "hedge",
            AccountPurpose::Reserve => "reserve",
        }
    }
}

/// Liquidation risk levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LiquidationRisk {
    /// Health factor > 2.0 (Safe)
    Safe,
    /// Health factor 1.5-2.0 (Monitor closely)
    Warning,
    /// Health factor 1.2-1.5 (High risk)
    Critical,
    /// Health factor < 1.2 (Emergency)
    Emergency,
}

impl LiquidationRisk {
    pub fn from_health_factor(health_factor: f64) -> Self {
        match health_factor {
            hf if hf > 2.0 => LiquidationRisk::Safe,
            hf if hf > 1.5 => LiquidationRisk::Warning,
            hf if hf > 1.2 => LiquidationRisk::Critical,
            _ => LiquidationRisk::Emergency,
        }
    }

    pub fn is_emergency(&self) -> bool {
        matches!(self, LiquidationRisk::Emergency)
    }

    pub fn requires_action(&self) -> bool {
        matches!(
            self,
            LiquidationRisk::Critical | LiquidationRisk::Emergency
        )
    }
}

/// Trading account configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TradingAccount {
    /// Unique account identifier
    pub id: String,

    /// Protocol this account trades on
    pub protocol: Protocol,

    /// Public key/address on the protocol
    pub public_key: String,

    /// Purpose/strategy for this account
    pub purpose: AccountPurpose,

    /// Current leverage setting
    pub current_leverage: f64,

    /// Capital allocation (0.0 to 1.0)
    pub capital_allocation: f64,

    /// Maximum position size as % of account
    pub max_position_size: f64,

    /// Stop loss percentage
    pub stop_loss_percent: f64,

    /// Take profit percentage
    pub take_profit_percent: f64,

    /// Whether account is actively trading
    pub is_active: bool,

    /// Creation timestamp
    pub created_at: i64,

    /// Last updated timestamp
    pub updated_at: i64,
}

impl TradingAccount {
    /// Create a new trading account
    pub fn new(
        id: String,
        protocol: Protocol,
        public_key: String,
        purpose: AccountPurpose,
    ) -> Self {
        let now = chrono::Utc::now().timestamp();
        let leverage = purpose.max_leverage();

        Self {
            id,
            protocol,
            public_key,
            purpose,
            current_leverage: leverage,
            capital_allocation: 0.0,
            max_position_size: 0.10,
            stop_loss_percent: 0.02,
            take_profit_percent: 0.05,
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }

    /// Validate account configuration
    pub fn validate(&self) -> Result<()> {
        use crate::utils::Error;

        if self.capital_allocation < 0.0 || self.capital_allocation > 1.0 {
            return Err(Error::InvalidAccountConfig);
        }

        if self.max_position_size < 0.0 || self.max_position_size > 0.5 {
            return Err(Error::InvalidAccountConfig);
        }

        if self.current_leverage > self.purpose.max_leverage() {
            return Err(Error::InvalidAccountConfig);
        }

        if self.current_leverage < 0.0 {
            return Err(Error::InvalidAccountConfig);
        }

        Ok(())
    }

    /// Update timestamp
    pub fn touch(&mut self) {
        self.updated_at = chrono::Utc::now().timestamp();
    }
}

impl Default for TradingAccount {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            protocol: Protocol::Drift,
            public_key: String::new(),
            purpose: AccountPurpose::Scalp,
            current_leverage: 100.0,
            capital_allocation: 0.0,
            max_position_size: 0.10,
            stop_loss_percent: 0.02,
            take_profit_percent: 0.05,
            is_active: true,
            created_at: chrono::Utc::now().timestamp(),
            updated_at: chrono::Utc::now().timestamp(),
        }
    }
}

/// Account health metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthMetrics {
    /// Account ID
    pub account_id: String,

    /// Total equity in USDC
    pub total_equity: f64,

    /// Maintenance margin requirement
    pub maintenance_margin: f64,

    /// Health factor (equity / maintenance_margin)
    pub health_factor: f64,

    /// Liquidation risk level
    pub liquidation_risk: LiquidationRisk,

    /// Timestamp of measurement
    pub timestamp: i64,
}

impl HealthMetrics {
    /// Create new health metrics
    pub fn new(
        account_id: String,
        total_equity: f64,
        maintenance_margin: f64,
    ) -> Self {
        let health_factor = if maintenance_margin > 0.0 {
            total_equity / maintenance_margin
        } else {
            f64::INFINITY
        };

        let liquidation_risk = LiquidationRisk::from_health_factor(health_factor);

        Self {
            account_id,
            total_equity,
            maintenance_margin,
            health_factor,
            liquidation_risk,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }

    /// Check if account can trade
    pub fn can_trade(&self) -> bool {
        self.health_factor > 1.5 && !matches!(self.liquidation_risk, LiquidationRisk::Emergency)
    }

    /// Check if emergency action needed
    pub fn is_emergency(&self) -> bool {
        self.liquidation_risk.is_emergency()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_purpose_leverage() {
        assert_eq!(AccountPurpose::Scalp.max_leverage(), 100.0);
        assert_eq!(AccountPurpose::Swing.max_leverage(), 20.0);
        assert_eq!(AccountPurpose::Position.max_leverage(), 10.0);
        assert_eq!(AccountPurpose::Hedge.max_leverage(), 5.0);
        assert_eq!(AccountPurpose::Reserve.max_leverage(), 0.0);
    }

    #[test]
    fn test_trading_account_creation() {
        let account = TradingAccount::new(
            "test-1".to_string(),
            Protocol::Drift,
            "key123".to_string(),
            AccountPurpose::Scalp,
        );

        assert_eq!(account.id, "test-1");
        assert_eq!(account.protocol, Protocol::Drift);
        assert_eq!(account.purpose, AccountPurpose::Scalp);
        assert_eq!(account.current_leverage, 100.0);
        assert!(account.is_active);
    }

    #[test]
    fn test_trading_account_validation() {
        let mut account = TradingAccount::new(
            "test-1".to_string(),
            Protocol::Drift,
            "key123".to_string(),
            AccountPurpose::Scalp,
        );

        account.capital_allocation = 0.3;
        assert!(account.validate().is_ok());

        // Invalid allocation
        account.capital_allocation = 1.5;
        assert!(account.validate().is_err());

        // Invalid position size
        account.capital_allocation = 0.3;
        account.max_position_size = 0.6;
        assert!(account.validate().is_err());

        // Invalid leverage
        account.max_position_size = 0.2;
        account.current_leverage = 150.0;
        assert!(account.validate().is_err());
    }

    #[test]
    fn test_liquidation_risk_from_health_factor() {
        assert_eq!(
            LiquidationRisk::from_health_factor(3.0),
            LiquidationRisk::Safe
        );
        assert_eq!(
            LiquidationRisk::from_health_factor(1.7),
            LiquidationRisk::Warning
        );
        assert_eq!(
            LiquidationRisk::from_health_factor(1.3),
            LiquidationRisk::Critical
        );
        assert_eq!(
            LiquidationRisk::from_health_factor(1.0),
            LiquidationRisk::Emergency
        );
    }

    #[test]
    fn test_health_metrics_can_trade() {
        let metrics_safe = HealthMetrics::new("acc1".to_string(), 10000.0, 3000.0);
        assert!(metrics_safe.can_trade()); // health_factor = 3.33

        let metrics_warning = HealthMetrics::new("acc2".to_string(), 1700.0, 1000.0);
        assert!(!metrics_warning.can_trade()); // health_factor = 1.7, but need > 1.5

        let metrics_critical = HealthMetrics::new("acc3".to_string(), 1300.0, 1000.0);
        assert!(!metrics_critical.can_trade()); // health_factor = 1.3
    }

    #[test]
    fn test_account_touch() {
        let mut account = TradingAccount::default();
        let original_updated = account.updated_at;

        std::thread::sleep(std::time::Duration::from_millis(10));
        account.touch();

        assert!(account.updated_at > original_updated);
    }

    #[test]
    fn test_protocol_as_str() {
        assert_eq!(Protocol::Drift.as_str(), "drift");
        assert_eq!(Protocol::Hyperliquid.as_str(), "hyperliquid");
        assert_eq!(Protocol::Phantom.as_str(), "phantom");
    }

    #[test]
    fn test_account_purpose_as_str() {
        assert_eq!(AccountPurpose::Scalp.as_str(), "scalp");
        assert_eq!(AccountPurpose::Swing.as_str(), "swing");
        assert_eq!(AccountPurpose::Position.as_str(), "position");
        assert_eq!(AccountPurpose::Hedge.as_str(), "hedge");
        assert_eq!(AccountPurpose::Reserve.as_str(), "reserve");
    }
}
