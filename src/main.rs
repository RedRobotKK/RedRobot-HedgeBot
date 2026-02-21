/// CLI entry point for the multi-protocol trading bot
use drift_multi_protocol::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .compact()
        .init();

    tracing::info!("🚀 Drift Multi-Protocol Trading Bot v{}", VERSION);
    tracing::info!("Initializing account manager...");

    // Create account manager
    let mut manager = AccountManager::new();

    // Set up trading accounts
    let accounts = vec![
        (
            "drift-scalp-1",
            Protocol::Drift,
            AccountPurpose::Scalp,
            0.30,
        ),
        (
            "drift-swing-1",
            Protocol::Drift,
            AccountPurpose::Swing,
            0.25,
        ),
        (
            "drift-position-1",
            Protocol::Drift,
            AccountPurpose::Position,
            0.20,
        ),
        (
            "drift-hedge-1",
            Protocol::Drift,
            AccountPurpose::Hedge,
            0.15,
        ),
        (
            "drift-reserve-1",
            Protocol::Drift,
            AccountPurpose::Reserve,
            0.10,
        ),
    ];

    tracing::info!("Registering accounts...");
    for (id, protocol, purpose, allocation) in accounts {
        let mut account = TradingAccount::new(
            id.to_string(),
            protocol,
            format!("public_key_{}", id),
            purpose,
        );
        account.capital_allocation = allocation;

        match manager.register_account(account) {
            Ok(registered_id) => {
                tracing::info!("✅ Account registered: {}", registered_id);
            }
            Err(e) => {
                tracing::error!("❌ Failed to register account: {}", e);
            }
        }
    }

    // Display account information
    tracing::info!("\n📊 Account Summary:");
    tracing::info!("Total Accounts: {}", manager.total_account_count());
    tracing::info!("Active Accounts: {}", manager.active_account_count());
    tracing::info!("Total Capital Allocated: {:.1}%", manager.total_capital_allocated() * 100.0);

    tracing::info!("\n📋 Account Details:");
    for summary in manager.get_all_account_summaries() {
        tracing::info!(
            "  {} | Protocol: {:?} | Purpose: {:?} | Leverage: {:.1}x | Capital: {:.1}%",
            summary.id,
            summary.protocol,
            summary.purpose,
            summary.current_leverage,
            summary.capital_allocation * 100.0
        );
    }

    // Demonstrate rebalancing
    tracing::info!("\n⚙️ Testing rebalancing...");
    if let Ok(_) = manager.set_capital_allocation("drift-scalp-1", 0.40) {
        if let Ok(_) = manager.set_capital_allocation("drift-swing-1", 0.15) {
            tracing::info!("✅ Rebalancing complete");
            tracing::info!("New total allocation: {:.1}%", manager.total_capital_allocated() * 100.0);
        }
    }

    // Demonstrate account filtering
    tracing::info!("\n🔍 Drift Protocol Accounts:");
    for account in manager.get_accounts_by_protocol(Protocol::Drift) {
        tracing::info!("  {} | {}", account.id, account.public_key);
    }

    tracing::info!("\n🎯 Scalp Trading Accounts:");
    for account in manager.get_accounts_by_purpose(AccountPurpose::Scalp) {
        tracing::info!("  {} | Leverage: {:.1}x", account.id, account.current_leverage);
    }

    tracing::info!("\n✅ Bot initialized and ready for trading!");
    tracing::info!("Next steps:");
    tracing::info!("  1. Connect to Drift Protocol");
    tracing::info!("  2. Integrate with Hyperliquid");
    tracing::info!("  3. Deploy capital management system");
    tracing::info!("  4. Enable liquidation prevention");

    Ok(())
}
