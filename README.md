# lbank-rs

[![Crates.io](https://img.shields.io/crates/v/lbank-rs.svg)](https://crates.io/crates/lbank-rs)
[![Documentation](https://docs.rs/lbank-rs/badge.svg)](https://docs.rs/lbank-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Rust library for [LBank](https://www.lbank.com) cryptocurrency exchange API.

## Features

- ✅ **Dual client support**: Both blocking and async HTTP clients
- ✅ **Unified API**: Single method works with both client types
- ✅ **Type-safe**: Full Rust type safety with proper error handling
- ✅ **Complete coverage**: All LBank REST API v2 endpoints
- ✅ **Easy to use**: Simple, ergonomic API design
- ✅ **Configurable**: Custom endpoints and settings

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
lbank-rs = "0.2"

# For async support
tokio = { version = "1", features = ["full"] }
```

## Quick Start

### Blocking Client

```rust
use lbank_rs::{api::LBank, general::General};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a blocking client
    let general = General::new(None, None);
    
    // Ping the server
    let response = general.ping()?;
    println!("Ping response: {}", response);
    
    Ok(())
}
```

### Async Client

```rust
use lbank_rs::{api::LBank, general::General};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an async client
    let general = General::new_async(None, None);
    
    // Ping the server
    let response = general.ping()?;
    println!("Ping response: {}", response);
    
    Ok(())
}
```

### With API Keys

```rust
use lbank_rs::{api::LBank, general::General};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = Some("your-api-key".to_string());
    let secret_key = Some("your-secret-key".to_string());
    
    let general = General::new(api_key, secret_key);
    
    // Now you can call authenticated endpoints
    // let account = general.get_account_info()?;
    
    Ok(())
}
```

### Custom Configuration

```rust
use lbank_rs::{api::LBank, config::Config, general::General};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Config::default();
    config.rest_api_endpoint = "https://api.lbank.com".to_string();
    
    let general = General::new_with_config(None, None, &config);
    
    Ok(())
}
```

## API Coverage

### Market Data (Public)

- System ping
- Server time
- Exchange info
- Market depth
- Recent trades
- Kline/candlestick data
- 24hr ticker statistics
- Price ticker
- Order book ticker

### Account & Trading (Authenticated)

- Account information
- Create order
- Cancel order
- Query order
- Open orders
- Order history
- Trade history

### Wallet

- Deposit history
- Withdraw
- Deposit address
- Asset details

## Blocking vs Async

### When to use Blocking:

- Simple scripts and tools
- Single sequential API calls
- When you don't need concurrency

### When to use Async:

- High-performance applications
- Multiple concurrent requests
- Server applications
- Real-time data streaming

## Concurrent Requests (Async)

```rust
use futures::future::join_all;
use lbank_rs::{api::LBank, general::General};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let general = General::new_async(None, None);
    
    // Make multiple requests concurrently
    let futures = vec![
        general.ping_async(),
        general.ping_async(),
        general.ping_async(),
    ];
    
    let results = join_all(futures).await;
    
    // All requests completed in parallel!
    
    Ok(())
}
```

## Error Handling

The library uses `error_chain` for comprehensive error handling:

```rust
use lbank_rs::{api::LBank, general::General, errors::Result};

fn main() -> Result<()> {
    let general = General::new(None, None);
    
    match general.ping() {
        Ok(response) => println!("Success: {}", response),
        Err(e) => {
            eprintln!("Error: {}", e);
            // You can also inspect the error chain
            for err in e.iter() {
                eprintln!("  caused by: {}", err);
            }
        }
    }
    
    Ok(())
}
```

## Verbose Logging

Enable verbose mode to see request details:

```rust
use lbank_rs::{api::LBank, general::General};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut general = General::new(None, None);
    general.set_verbose(true);
    
    // Now you'll see request URLs and bodies
    let response = general.ping()?;
    
    Ok(())
}
```

## Documentation

- [LBank API Documentation](https://www.lbank.com/en-US/docs/index.html)
- [API Rust Docs](https://docs.rs/lbank-rs)

## Examples

Check the `examples/` directory for more usage examples:

```bash
cargo run --example ping_test
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Disclaimer

This software is for educational purposes only. Use at your own risk. The authors and contributors are not responsible for any financial losses incurred through the use of this library.

## Links

- [Crates.io](https://crates.io/crates/lbank-rs)
- [Documentation](https://docs.rs/lbank-rs)
- [LBank Exchange](https://www.lbank.com)
- [LBank API Docs](https://www.lbank.com/en-US/docs/index.html)

