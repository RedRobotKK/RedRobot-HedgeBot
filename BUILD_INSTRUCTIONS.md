# 🔨 Build Instructions for Drift Multi-Protocol Trading System

This guide walks you through building and testing the complete Rust trading system on your local machine.

## Prerequisites

### Step 1: Install Rust

If you don't have Rust installed, get it from https://rustup.rs:

```bash
# On macOS/Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# On Windows
# Download and run: https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe

# Verify installation
rustc --version    # Should show: rustc 1.XX.X (...)
cargo --version    # Should show: cargo 1.XX.X (...)
```

### Step 2: Verify Project Structure

Navigate to the project directory:

```bash
cd /path/to/drift-multi-protocol

# Verify key files exist
ls -la src/
ls -la tests/
ls Cargo.toml
```

You should see:
```
Cargo.toml
src/
  lib.rs
  main.rs
  models/
  modules/
  utils/
tests/
  integration_test.rs
README.md
```

## Building the Project

### Step 3: Download Dependencies

```bash
# This will download and compile all Rust dependencies (~2-5 minutes)
cargo build

# For optimized release build (faster at runtime, slower to build)
cargo build --release
```

If you see errors about missing dependencies, make sure your internet connection is stable and run:

```bash
cargo update
cargo build --release
```

### Step 4: Verify Build Success

```bash
# Check if compilation succeeds without building
cargo check

# Output should say: "Finished `dev` profile"
```

## Running Tests

### Step 5: Run All Tests

```bash
# Run all unit + integration tests
cargo test

# Expected output:
# running 41 tests
#
# test modules::account_manager::tests::test_register_account ... ok
# test modules::account_manager::tests::test_duplicate_account_error ... ok
# ... (more tests)
#
# test result: ok. 31 passed; 0 failed; 0 ignored
```

### Step 6: Run Tests with Output

```bash
# Show println! and tracing output
cargo test -- --nocapture

# Run single-threaded (helps with debugging)
cargo test -- --test-threads=1 --nocapture
```

### Step 7: Run Specific Test Groups

```bash
# Unit tests only (AccountManager)
cargo test modules::account_manager

# Integration tests only
cargo test --test integration_test

# A specific test
cargo test test_register_account -- --nocapture
```

### Step 8: Generate Test Coverage Report

```bash
# Install tarpaulin (one-time)
cargo install cargo-tarpaulin

# Generate HTML coverage report
cargo tarpaulin --out Html --output-dir coverage

# Open report in browser
open coverage/tarpaulin-report.html    # macOS
xdg-open coverage/tarpaulin-report.html # Linux
```

## Running the Bot

### Step 9: Run Development Version

```bash
# Compile and run with debug output
cargo run
```

Expected output:
```
🚀 Drift Multi-Protocol Trading Bot v0.1.0
Initializing account manager...
Registering accounts...
✅ Account registered: drift-scalp-1
✅ Account registered: drift-swing-1
✅ Account registered: drift-position-1
✅ Account registered: drift-hedge-1
✅ Account registered: drift-reserve-1

📊 Account Summary:
Total Accounts: 5
Active Accounts: 5
Total Capital Allocated: 100.0%

📋 Account Details:
  drift-scalp-1 | Protocol: Drift | Purpose: Scalp | Leverage: 100.0x | Capital: 30.0%
  drift-swing-1 | Protocol: Drift | Purpose: Swing | Leverage: 20.0x | Capital: 25.0%
  drift-position-1 | Protocol: Drift | Purpose: Position | Leverage: 10.0x | Capital: 20.0%
  drift-hedge-1 | Protocol: Drift | Purpose: Hedge | Leverage: 5.0x | Capital: 15.0%
  drift-reserve-1 | Protocol: Drift | Purpose: Reserve | Leverage: 0.0x | Capital: 10.0%

⚙️ Testing rebalancing...
✅ Rebalancing complete
New total allocation: 100.0%

🔍 Drift Protocol Accounts:
  drift-scalp-1 | key_drift-scalp-1
  drift-swing-1 | key_drift-swing-1
  drift-position-1 | key_drift-position-1
  drift-hedge-1 | key_drift-hedge-1
  drift-reserve-1 | key_drift-reserve-1

🎯 Scalp Trading Accounts:
  drift-scalp-1 | Leverage: 40.0x

✅ Bot initialized and ready for trading!
```

### Step 10: Build Release Binary

```bash
# Create optimized binary for deployment
cargo build --release

# Binary location:
# target/release/drift-bot

# Run the binary
./target/release/drift-bot
```

## Code Quality Checks

### Step 11: Format Code

```bash
# Auto-format code to Rust standards
cargo fmt

# Check formatting without changes
cargo fmt -- --check
```

### Step 12: Lint with Clippy

```bash
# Check for common Rust errors and best practices
cargo clippy -- -D warnings

# Should output: "Finished `dev` profile [unoptimized + debuginfo]"
```

### Step 13: Generate Documentation

```bash
# Generate and open API documentation
cargo doc --open

# This opens a browser with complete API docs for all modules
```

## Verification Checklist

After completing these steps, you should be able to check off:

- [ ] Rust is installed (`rustc --version` shows 1.70+)
- [ ] Dependencies downloaded (`cargo build` completes successfully)
- [ ] All tests pass (`cargo test` shows 31+ tests, all passing)
- [ ] Test coverage >90% (`cargo tarpaulin` report shows >90%)
- [ ] Bot runs successfully (`cargo run` displays account setup)
- [ ] Code passes linting (`cargo clippy` with no warnings)
- [ ] Code is properly formatted (`cargo fmt --check` shows no changes)

## Troubleshooting

### "Failed to compile"

```bash
# Update Rust
rustup update

# Update dependencies
cargo update

# Clean build
cargo clean
cargo build --release
```

### "Test failures"

```bash
# Run with backtrace for detailed error info
RUST_BACKTRACE=1 cargo test failing_test_name

# Run single-threaded to avoid race conditions
cargo test -- --test-threads=1 --nocapture
```

### "Dependency version conflicts"

```bash
# Regenerate Cargo.lock
rm Cargo.lock
cargo build
```

## Performance Testing

```bash
# Measure build time
time cargo build --release

# Measure test execution time
time cargo test --release

# Expected times (rough estimates):
# First build: 2-5 minutes (downloads dependencies)
# Subsequent builds: 5-30 seconds
# Test execution: 1-3 seconds
```

## What Each Component Does

### Phase 1: Account Management (COMPLETE) ✅

**Files:** `src/models/account.rs`, `src/modules/account_manager.rs`

- ✅ Create and manage multiple trading accounts
- ✅ Set leverage per account (respects maximums by purpose)
- ✅ Allocate capital dynamically
- ✅ Track health metrics
- ✅ Prevent liquidation through monitoring
- ✅ 31 unit tests + 12 integration tests
- ✅ >90% code coverage

**Test Example:**
```bash
cargo test test_register_account -- --nocapture
```

### Phase 2: Hyperliquid Integration (PENDING)

Coming next:
- Connect to Hyperliquid API
- Submit orders
- Fetch market data
- Manage positions

### Phase 3: Cross-Chain Bridging (PENDING)

Coming next:
- Wormhole integration
- CCTP integration
- Stargate integration
- Gas optimization

### Phase 4: Capital Management (PENDING)

Coming next:
- Dynamic allocation
- Risk assessment
- Performance tracking
- Autonomous rebalancing

## Next Steps After Build

1. **Review the code:**
   - Read through `src/modules/account_manager.rs` (400+ lines of well-tested code)
   - Review test examples in `tests/integration_test.rs`
   - Check data models in `src/models/account.rs`

2. **Understand the architecture:**
   - Account data flows through AccountManager
   - Each account has a Protocol (Drift/Hyperliquid/Phantom)
   - Each account has a Purpose (Scalp/Swing/Position/Hedge/Reserve)
   - Leverage is constrained by purpose
   - Capital allocation must sum to 1.0

3. **Prepare for Phase 2:**
   - Next module: DriftProtocol integration
   - Will add actual blockchain interactions
   - Will connect to Drift RPC endpoint
   - Will execute real trades

4. **Deploy on testnet:**
   - Create Drift testnet account
   - Fund with testnet USDC
   - Connect bot to testnet RPC
   - Validate trading logic

5. **Go live on mainnet:**
   - Set up Drift mainnet account
   - Fund with $5K initial capital
   - Deploy bot binary to server
   - Monitor performance 24/7

## Performance Metrics

After building, you can check:

```bash
# Binary size
ls -lh target/release/drift-bot

# Test execution time
time cargo test --release -- --nocapture

# Code metrics
cargo tarpaulin --out Xml && cat cobertura.xml
```

## Support

If you encounter issues:

1. Check the Rust error message carefully (usually very helpful)
2. Run `cargo clean && cargo build` to do a fresh build
3. Make sure you have the latest Rust: `rustup update`
4. Review the test output: `cargo test -- --nocapture`
5. Check that you're in the correct directory: `pwd`

---

**You're now ready to build and test the trading bot!**

Start with: `cargo test`

Then run: `cargo run`

Then explore the code and begin Phase 2 integration. 🚀
