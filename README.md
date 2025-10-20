# lbank-rs

[![Crates.io](https://img.shields.io/crates/v/lbank-rs.svg)](https://crates.io/crates/lbank-rs)
[![Documentation](https://docs.rs/lbank-rs/badge.svg)](https://docs.rs/lbank-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Complete Rust SDK for [LBank](https://www.lbank.com) cryptocurrency exchange API.

## Features

- ✅ **Complete API Coverage**: All 32 LBank REST API v2 endpoints
- ✅ **Dual Client Support**: Both blocking and async HTTP clients  
- ✅ **Type-Safe**: Full Rust type safety with comprehensive error handling
- ✅ **Authenticated Trading**: RSA and HmacSHA256 signature support
- ✅ **Modular Design**: Separate modules for Market, Trading, Wallet, and Account
- ✅ **Easy Configuration**: Dotenv support and custom endpoint configuration
- ✅ **Well Documented**: Complete examples for every module

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
lbank-rs = "0.3"
tokio = { version = "1", features = ["full"] }  # For async support
dotenv = "0.15"  # Optional: for loading API credentials
```

## Quick Start

### Public Market Data (No Auth Required)

```rust
use lbank_rs::{market::Market, client::Client};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(None, None);
    let market = Market { client };
    
    // Get market depth
    let depth = market.depth("lbk_usdt", 10)?;
    println!("Market depth: {}", depth);
    
    // Get 24hr ticker
    let ticker = market.ticker_24hr("lbk_usdt")?;
    println!("24hr ticker: {}", ticker);
    
    Ok(())
}
```

### Authenticated Trading

```rust
use lbank_rs::{spot::Spot, client::Client};
use dotenv::dotenv;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    
    let api_key = env::var("LBANK_API_KEY").ok();
    let secret_key = env::var("LBANK_SECRET_KEY").ok();
    
    let client = Client::new(api_key, secret_key);
    let spot = Spot { client };
    
    // Get account information
    let account = spot.account_info()?;
    println!("Account: {}", account);
    
    // Test order (safe - doesn't place real order)
    let test_order = spot.create_order_test(
        "lbk_usdt",
        "buy",
        Some("0.01"),
        Some("100"),
        None,
        None
    )?;
    println!("Order test: {}", test_order);
    
    Ok(())
}
```

### Async Client

```rust
use lbank_rs::{market::AsyncMarket, client::AsyncClient};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AsyncClient::new(None, None);
    let market: Arc<AsyncMarket> = Arc::new(AsyncMarket { client });
    
    // Spawn concurrent requests
    let market1 = market.clone();
    let handle1 = tokio::spawn(async move {
        market1.ticker_24hr("lbk_usdt").await
    });
    
    let market2 = market.clone();
    let handle2 = tokio::spawn(async move {
        market2.depth("eth_usdt", 10).await
    });
    
    // Wait for all requests
    let (ticker, depth) = tokio::try_join!(handle1, handle2)?;
    
    println!("Ticker: {:?}", ticker?);
    println!("Depth: {:?}", depth?);
    
    Ok(())
}
```

## API Modules

### 1. Common (General) - 5 endpoints
Public configuration and system information:
- `currency_pairs()` - Get all trading pairs
- `accuracy()` - Get price/quantity precision for pairs
- `asset_configs()` - Get deposit/withdrawal configs
- `time()` - Get server timestamp

### 2. Market Data - 8 endpoints
Public market information:
- `system_ping()` - Test connectivity
- `depth(symbol, size)` - Order book depth
- `price(symbol?)` - Latest prices
- `book_ticker(symbol)` - Best bid/ask
- `ticker_24hr(symbol)` - 24hr statistics
- `etf_ticker_24hr(symbol)` - ETF ticker
- `trades(symbol, size, time?)` - Recent trades
- `kline(symbol, size, type, time)` - Candlestick data

### 3. Wallet - 7 endpoints
Deposit and withdrawal management (requires authentication):
- `system_status()` - System maintenance status
- `user_info()` - All coin balances
- `withdraw(...)` - Submit withdrawal
- `deposit_history(...)` - Deposit records
- `withdraw_history(...)` - Withdrawal records
- `deposit_address(coin, network?)` - Get deposit address
- `asset_detail(coin?)` - Asset information

### 4. Spot Trading - 9 endpoints
Order and trade management (requires authentication):
- `create_order_test(...)` - Test order (safe)
- `create_order(...)` - Place order
- `cancel_order(...)` - Cancel specific order
- `cancel_order_by_symbol(symbol)` - Cancel all orders
- `order_info(...)` - Query order details
- `open_orders(...)` - Current pending orders
- `order_history(...)` - Historical orders
- `account_info()` - Account balances
- `transaction_history(...)` - Trade history

### 5. Account - 3 endpoints
Account settings and permissions (requires authentication):
- `trade_fee_rate(category?)` - Trading fees
- `api_restrictions()` - API key permissions
- `account_info()` - Account information

## Authentication

LBank supports two signature methods:

### 1. RSA Signature (Recommended)
```bash
# In your .env file
LBANK_API_KEY=your_api_key
LBANK_SECRET_KEY=-----BEGIN PRIVATE KEY-----
MIIEvQIBADANBgkqhkiG9w0BAQEFAASC...
-----END PRIVATE KEY-----
```

### 2. HmacSHA256 Signature
```bash
# In your .env file
LBANK_API_KEY=your_api_key
LBANK_SECRET_KEY=your_hex_secret_key
```

The library automatically detects which signature method to use based on the secret key format.

## Environment Setup

Create a `.env` file in your project root:

```bash
LBANK_API_KEY=your_api_key_here
LBANK_SECRET_KEY=your_secret_key_here
```

Then use dotenv in your code:

```rust
use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let api_key = env::var("LBANK_API_KEY").ok();
    let secret_key = env::var("LBANK_SECRET_KEY").ok();
    
    // Use credentials...
}
```

## Order Types

The library supports all LBank order types:

- `buy` / `sell` - Limit orders
- `buy_market` / `sell_market` - Market orders
- `buy_maker` / `sell_maker` - Post-only orders
- `buy_ioc` / `sell_ioc` - Immediate-or-cancel
- `buy_fok` / `sell_fok` - Fill-or-kill

## Error Handling

```rust
use lbank_rs::errors::Result;

fn trade() -> Result<()> {
    let client = Client::new(None, None);
    let spot = Spot { client };
    
    match spot.account_info() {
        Ok(info) => println!("Account: {}", info),
        Err(e) => {
            eprintln!("Error: {}", e);
            // Handle specific error types
            for cause in e.iter() {
                eprintln!("Caused by: {}", cause);
            }
        }
    }
    
    Ok(())
}
```

## Examples

The repository includes comprehensive examples for each module:

```bash
# Market data examples
cargo run --example market_endpoints_test
cargo run --example market_endpoints_async_test

# Wallet examples
cargo run --example wallet_endpoints_test
cargo run --example wallet_endpoints_async_test

# Spot trading examples
cargo run --example spot_endpoints_test
cargo run --example spot_endpoints_async_test

# Account examples
cargo run --example account_endpoints_test
cargo run --example account_endpoints_async_test

# General endpoints
cargo run --example general_endpoints_blocking_test
cargo run --example general_endpoints_async_test
```

## Custom Configuration

```rust
use lbank_rs::{config::Config, client::Client};

let mut config = Config::default();
config.rest_api_endpoint = "https://api.lbank.info".to_string();

let client = Client::new_with_config(
    Some(api_key),
    Some(secret_key),
    &config
);
```

## Rate Limits

LBank API has the following rate limits:
- **Order operations**: 500 requests per 10 seconds
- **Other endpoints**: 200 requests per 10 seconds

The async client is ideal for staying within rate limits while maximizing throughput.

## Safety Features

- 🔒 **Test Endpoints**: Use `create_order_test()` to validate orders without placing them
- ⚠️ **Clear Warnings**: Examples include safety warnings for destructive operations
- 🛡️ **Type Safety**: Rust's type system prevents common API usage errors
- ✅ **Signature Verification**: Automatic signature generation for authenticated requests

## Documentation

- [LBank API Documentation](https://www.lbank.com/en-US/docs/index.html)
- [Rust API Docs](https://docs.rs/lbank-rs)
- [Repository Examples](https://github.com/mmingyeomm/lbank-rs/tree/master/examples)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Changelog

### Version 0.3.0 (2025-01-XX)
- ✨ Complete API coverage: All 32 REST endpoints
- ✨ Added Market module with 8 endpoints
- ✨ Added Wallet module with 7 endpoints  
- ✨ Added Spot Trading module with 9 endpoints
- ✨ Added Account module with 3 endpoints
- ✨ Full RSA and HmacSHA256 signature support
- ✨ Comprehensive examples for all modules
- ✨ Arc + tokio::spawn pattern for async tests
- 📚 Complete documentation overhaul

### Version 0.2.0
- Initial release with basic functionality

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Disclaimer

**This software is for educational and development purposes only. Use at your own risk.**

- The authors and contributors are not responsible for any financial losses
- Always test with small amounts first
- Use `create_order_test()` before placing real orders
- Keep your API keys secure and never commit them to version control

## Links

- [Crates.io](https://crates.io/crates/lbank-rs)
- [Documentation](https://docs.rs/lbank-rs)
- [GitHub Repository](https://github.com/mmingyeomm/lbank-rs)
- [LBank Exchange](https://www.lbank.com)
- [LBank API Documentation](https://www.lbank.com/en-US/docs/index.html)

---

Made with ❤️ for the Rust and crypto communities
