use lbank_rs::{wallet::Wallet, client::Client};
use dotenv::dotenv;
use std::env;

/// Example for testing Wallet endpoints
/// 
/// NOTE: Most wallet endpoints require authentication with valid API credentials.
/// Create a .env file with your credentials or set environment variables:
/// 
/// .env file:
///   LBANK_API_KEY=your_api_key
///   LBANK_SECRET_KEY=your_secret_key
/// 
/// Or export:
///   export LBANK_API_KEY="your_api_key"
///   export LBANK_SECRET_KEY="your_secret_key"
/// 
/// Usage:
///   cargo run --example wallet_endpoints_test
/// 
/// WARNING: Be careful with withdraw() function - it performs real withdrawals!

fn main() {
    // Load .env file if it exists
    dotenv().ok();
    
    println!("\n=== Testing Synchronous Wallet Endpoints ===\n");
    
    // Get API credentials from environment variables
    let api_key = env::var("LBANK_API_KEY").ok();
    let secret_key = env::var("LBANK_SECRET_KEY").ok();

    // Create wallet client
    let client = if api_key.is_some() && secret_key.is_some() {
        Client::new(api_key.clone(), secret_key.clone())
    } else {
        println!("⚠️  Warning: No API credentials found in environment");
        println!("   Set LBANK_API_KEY and LBANK_SECRET_KEY to test authenticated endpoints\n");
        Client::new(None, None)
    };

    let wallet = Wallet { client };

    // Test system status (public endpoint - no auth required)
    println!("1. Testing system_status endpoint...");
    match wallet.system_status() {
        Ok(response) => println!("✓ System Status Response: {}\n", response),
        Err(e) => println!("✗ System Status Error: {}\n", e),
    }

    // Only test authenticated endpoints if credentials are provided
    if api_key.is_none() || secret_key.is_none() {
        println!("Skipping authenticated endpoints (no credentials provided)\n");
        println!("=== Completed Testing Wallet Endpoints ===\n");
        return;
    }

    // Test user info
    println!("2. Testing user_info endpoint...");
    match wallet.user_info() {
        Ok(response) => {
            let display = if response.len() > 500 {
                format!("{}... (truncated)", &response[..500])
            } else {
                response.clone()
            };
            println!("✓ User Info Response: {}\n", display);
        },
        Err(e) => println!("✗ User Info Error: {}\n", e),
    }

    // Test deposit history
    println!("3. Testing deposit_history endpoint (USDT, last 30 days)...");
    let end_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    let start_time = end_time - (30 * 24 * 60 * 60 * 1000); // 30 days ago

    match wallet.deposit_history(None, Some("usdt"), Some(start_time), Some(end_time)) {
        Ok(response) => {
            let display = if response.len() > 500 {
                format!("{}... (truncated)", &response[..500])
            } else {
                response.clone()
            };
            println!("✓ Deposit History Response: {}\n", display);
        },
        Err(e) => println!("✗ Deposit History Error: {}\n", e),
    }

    // Test withdrawal history
    println!("4. Testing withdraw_history endpoint (last 30 days)...");
    match wallet.withdraw_history(None, None, None, Some(start_time), Some(end_time)) {
        Ok(response) => {
            let display = if response.len() > 500 {
                format!("{}... (truncated)", &response[..500])
            } else {
                response.clone()
            };
            println!("✓ Withdrawal History Response: {}\n", display);
        },
        Err(e) => println!("✗ Withdrawal History Error: {}\n", e),
    }

    // Test deposit address
    println!("5. Testing deposit_address endpoint (USDT)...");
    match wallet.deposit_address("usdt", None) {
        Ok(response) => println!("✓ Deposit Address Response: {}\n", response),
        Err(e) => println!("✗ Deposit Address Error: {}\n", e),
    }

    // Test asset detail
    println!("6. Testing asset_detail endpoint (USDT)...");
    match wallet.asset_detail(Some("usdt")) {
        Ok(response) => println!("✓ Asset Detail Response: {}\n", response),
        Err(e) => println!("✗ Asset Detail Error: {}\n", e),
    }

    // Test asset detail for all coins
    println!("7. Testing asset_detail endpoint (all coins)...");
    match wallet.asset_detail(None) {
        Ok(response) => {
            let display = if response.len() > 500 {
                format!("{}... (truncated)", &response[..500])
            } else {
                response.clone()
            };
            println!("✓ All Asset Details Response: {}\n", display);
        },
        Err(e) => println!("✗ All Asset Details Error: {}\n", e),
    }

    println!("=== Completed Testing Synchronous Wallet Endpoints ===\n");
    println!("Note: withdraw() endpoint not tested for safety reasons");
    println!("      Use it carefully in production with proper validation!\n");
}

