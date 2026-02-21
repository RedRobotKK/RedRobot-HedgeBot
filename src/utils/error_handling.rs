/// Error types for the multi-protocol trading system
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum Error {
    #[error("Duplicate account ID")]
    DuplicateAccount,

    #[error("Account not found")]
    AccountNotFound,

    #[error("Invalid account configuration")]
    InvalidAccountConfig,

    #[error("Capital allocation does not sum to 1.0")]
    AllocationDoesNotSum,

    #[error("Invalid allocation value (must be between 0 and 1)")]
    InvalidAllocationValue,

    #[error("No viable trading opportunity")]
    NoViableOpportunity,

    #[error("Insufficient balance")]
    InsufficientBalance,

    #[error("Position size exceeds maximum")]
    PositionSizeExceeded,

    #[error("Account health too low for trading")]
    AccountHealthTooLow,

    #[error("Liquidation risk critical")]
    LiquidationCritical,

    #[error("Transfer failed")]
    TransferFailed,

    #[error("Bridge operation failed")]
    BridgeOperationFailed,

    #[error("Gas estimation failed")]
    GasEstimationFailed,

    #[error("API request failed: {0}")]
    ApiRequestFailed(String),

    #[error("Invalid bridge route")]
    InvalidBridgeRoute,

    #[error("Maximum daily loss exceeded")]
    DailyLossLimitExceeded,

    #[error("Stop loss triggered")]
    StopLossTriggered,

    #[error("Take profit triggered")]
    TakeProfitTriggered,

    #[error("Invalid protocol")]
    InvalidProtocol,

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Internal error: {0}")]
    InternalError(String),
}

impl Error {
    /// Check if error is recoverable
    pub fn is_recoverable(&self) -> bool {
        match self {
            Error::ApiRequestFailed(_) => true,
            Error::TransferFailed => true,
            Error::BridgeOperationFailed => true,
            Error::GasEstimationFailed => true,
            _ => false,
        }
    }

    /// Get error severity (1-10, where 10 is most critical)
    pub fn severity(&self) -> u8 {
        match self {
            Error::LiquidationCritical => 10,
            Error::DailyLossLimitExceeded => 9,
            Error::AccountHealthTooLow => 8,
            Error::PositionSizeExceeded => 7,
            Error::InsufficientBalance => 6,
            Error::ApiRequestFailed(_) => 4,
            Error::BridgeOperationFailed => 5,
            _ => 3,
        }
    }
}

/// Result type for operations
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_is_recoverable() {
        let api_error = Error::ApiRequestFailed("timeout".to_string());
        assert!(api_error.is_recoverable());

        let config_error = Error::InvalidAccountConfig;
        assert!(!config_error.is_recoverable());
    }

    #[test]
    fn test_error_severity() {
        assert_eq!(Error::LiquidationCritical.severity(), 10);
        assert_eq!(Error::DailyLossLimitExceeded.severity(), 9);
        assert_eq!(Error::InvalidAccountConfig.severity(), 3);
    }

    #[test]
    fn test_error_display() {
        let error = Error::DuplicateAccount;
        assert_eq!(error.to_string(), "Duplicate account ID");
    }
}
