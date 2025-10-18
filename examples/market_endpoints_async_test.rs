use std::sync::Arc;
use lbank_rs::{market::AsyncMarket, client::AsyncClient};

#[tokio::main]
async fn main() {
    println!("\n=== Testing Asynchronous Market Endpoints ===\n");
    
    // Create a new AsyncMarket instance without authentication (public endpoints)
    let client = AsyncClient::new(None, None);
    let async_market: Arc<AsyncMarket> = Arc::new(AsyncMarket { client });

    // Get current timestamp for kline test
    let current_timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Test system ping endpoint
    let market1 = async_market.clone();
    let handle1 = tokio::spawn(async move {
        println!("Testing system_ping endpoint...");
        match market1.system_ping().await {
            Ok(response) => println!("✓ System Ping Response: {}\n", response),
            Err(e) => println!("✗ System Ping Error: {}\n", e),
        }
    });

    // Test depth endpoint
    let market2 = async_market.clone();
    let handle2 = tokio::spawn(async move {
        println!("Testing depth endpoint (btc_usdt)...");
        match market2.depth("btc_usdt", 10).await {
            Ok(response) => {
                let display = if response.len() > 500 {
                    format!("{}... (truncated)", &response[..500])
                } else {
                    response.clone()
                };
                println!("✓ Depth Response: {}\n", display);
            },
            Err(e) => println!("✗ Depth Error: {}\n", e),
        }
    });

    // Test price endpoint (single pair)
    let market3 = async_market.clone();
    let handle3 = tokio::spawn(async move {
        println!("Testing price endpoint (btc_usdt)...");
        match market3.price(Some("btc_usdt")).await {
            Ok(response) => println!("✓ Price Response: {}\n", response),
            Err(e) => println!("✗ Price Error: {}\n", e),
        }
    });

    // Test price endpoint (all pairs)
    let market4 = async_market.clone();
    let handle4 = tokio::spawn(async move {
        println!("Testing price endpoint (all pairs)...");
        match market4.price(None).await {
            Ok(response) => {
                let display = if response.len() > 500 {
                    format!("{}... (truncated)", &response[..500])
                } else {
                    response.clone()
                };
                println!("✓ All Prices Response: {}\n", display);
            },
            Err(e) => println!("✗ All Prices Error: {}\n", e),
        }
    });

    // Test book ticker endpoint
    let market5 = async_market.clone();
    let handle5 = tokio::spawn(async move {
        println!("Testing book_ticker endpoint (btc_usdt)...");
        match market5.book_ticker("btc_usdt").await {
            Ok(response) => println!("✓ Book Ticker Response: {}\n", response),
            Err(e) => println!("✗ Book Ticker Error: {}\n", e),
        }
    });

    // Test 24hr ticker endpoint
    let market6 = async_market.clone();
    let handle6 = tokio::spawn(async move {
        println!("Testing ticker_24hr endpoint (btc_usdt)...");
        match market6.ticker_24hr("btc_usdt").await {
            Ok(response) => println!("✓ 24hr Ticker Response: {}\n", response),
            Err(e) => println!("✗ 24hr Ticker Error: {}\n", e),
        }
    });

    // Test ETF 24hr ticker endpoint
    let market7 = async_market.clone();
    let handle7 = tokio::spawn(async move {
        println!("Testing etf_ticker_24hr endpoint (btc3l_usdt)...");
        match market7.etf_ticker_24hr("btc3l_usdt").await {
            Ok(response) => println!("✓ ETF 24hr Ticker Response: {}\n", response),
            Err(e) => println!("✗ ETF 24hr Ticker Error: {}\n", e),
        }
    });

    // Test trades endpoint
    let market8 = async_market.clone();
    let handle8 = tokio::spawn(async move {
        println!("Testing trades endpoint (btc_usdt, size=5)...");
        match market8.trades("btc_usdt", 5, None).await {
            Ok(response) => {
                let display = if response.len() > 500 {
                    format!("{}... (truncated)", &response[..500])
                } else {
                    response.clone()
                };
                println!("✓ Recent Trades Response: {}\n", display);
            },
            Err(e) => println!("✗ Recent Trades Error: {}\n", e),
        }
    });

    // Test kline endpoint
    let market9 = async_market.clone();
    let handle9 = tokio::spawn(async move {
        println!("Testing kline endpoint (btc_usdt, 5min, size=10)...");
        match market9.kline("btc_usdt", 10, "minute5", current_timestamp).await {
            Ok(response) => {
                let display = if response.len() > 500 {
                    format!("{}... (truncated)", &response[..500])
                } else {
                    response.clone()
                };
                println!("✓ Kline Response: {}\n", display);
            },
            Err(e) => println!("✗ Kline Error: {}\n", e),
        }
    });

    // Join all handles
    let _ = tokio::try_join!(
        handle1,
        handle2,
        handle3,
        handle4,
        handle5,
        handle6,
        handle7,
        handle8,
        handle9,
    ).expect("Failed to join handles");

    println!("=== Completed Testing Asynchronous Market Endpoints ===\n");
}

