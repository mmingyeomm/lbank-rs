use lbank_rs::{market::Market, client::Client};

fn main() {
    println!("\n=== Testing Synchronous Market Endpoints ===\n");
    
    // Create a new Market instance without authentication (public endpoints)
    let client = Client::new(None, None);
    let market = Market { client };

    // Test system ping endpoint
    println!("Testing system_ping endpoint...");
    match market.system_ping() {
        Ok(response) => println!("✓ System Ping Response: {}\n", response),
        Err(e) => println!("✗ System Ping Error: {}\n", e),
    }

    // Test depth endpoint
    println!("Testing depth endpoint (btc_usdt)...");
    match market.depth("btc_usdt", 10) {
        Ok(response) => {
            // Truncate response if too long
            let display = if response.len() > 500 {
                format!("{}... (truncated)", &response[..500])
            } else {
                response.clone()
            };
            println!("✓ Depth Response: {}\n", display);
        },
        Err(e) => println!("✗ Depth Error: {}\n", e),
    }

    // Test price endpoint (single pair)
    println!("Testing price endpoint (btc_usdt)...");
    match market.price(Some("btc_usdt")) {
        Ok(response) => println!("✓ Price Response: {}\n", response),
        Err(e) => println!("✗ Price Error: {}\n", e),
    }

    // Test price endpoint (all pairs)
    println!("Testing price endpoint (all pairs)...");
    match market.price(None) {
        Ok(response) => {
            // Truncate response if too long
            let display = if response.len() > 500 {
                format!("{}... (truncated)", &response[..500])
            } else {
                response.clone()
            };
            println!("✓ All Prices Response: {}\n", display);
        },
        Err(e) => println!("✗ All Prices Error: {}\n", e),
    }

    // Test book ticker endpoint
    println!("Testing book_ticker endpoint (btc_usdt)...");
    match market.book_ticker("btc_usdt") {
        Ok(response) => println!("✓ Book Ticker Response: {}\n", response),
        Err(e) => println!("✗ Book Ticker Error: {}\n", e),
    }

    // Test 24hr ticker endpoint
    println!("Testing ticker_24hr endpoint (btc_usdt)...");
    match market.ticker_24hr("btc_usdt") {
        Ok(response) => println!("✓ 24hr Ticker Response: {}\n", response),
        Err(e) => println!("✗ 24hr Ticker Error: {}\n", e),
    }

    // Test ETF 24hr ticker endpoint
    println!("Testing etf_ticker_24hr endpoint (btc3l_usdt)...");
    match market.etf_ticker_24hr("btc3l_usdt") {
        Ok(response) => println!("✓ ETF 24hr Ticker Response: {}\n", response),
        Err(e) => println!("✗ ETF 24hr Ticker Error: {}\n", e),
    }

    // Test trades endpoint
    println!("Testing trades endpoint (btc_usdt, size=5)...");
    match market.trades("btc_usdt", 5, None) {
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

    // Test kline endpoint
    println!("Testing kline endpoint (btc_usdt, 5min, size=10)...");
    let current_timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    match market.kline("btc_usdt", 10, "minute5", current_timestamp) {
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

    println!("=== Completed Testing Synchronous Market Endpoints ===\n");
}

