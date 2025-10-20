use lbank_rs::{account::Account, client::Client};
use dotenv::dotenv;
use std::env;

/// Example for testing Account endpoints
/// 
/// NOTE: All account endpoints require authentication with valid API credentials.
/// Create a .env file with your credentials or set environment variables:
/// 
/// .env file:
///   LBANK_API_KEY=your_api_key
///   LBANK_SECRET_KEY=your_secret_key
/// 
/// Usage:
///   cargo run --example account_endpoints_test

fn main() {
    // Load .env file if it exists
    dotenv().ok();
    
    println!("\n=== Testing Synchronous Account Endpoints ===\n");
    
    // Get API credentials from environment variables
    let api_key = env::var("LBANK_API_KEY").ok();
    let secret_key = env::var("LBANK_SECRET_KEY").ok();

    if api_key.is_none() || secret_key.is_none() {
        println!("⚠️  Error: No API credentials found in environment");
        println!("   Set LBANK_API_KEY and LBANK_SECRET_KEY to test account endpoints\n");
        println!("   All account endpoints require authentication.\n");
        return;
    }

    // Create account client
    let client = Client::new(api_key, secret_key);
    let account = Account { client };

    // Test trade fee rate (all pairs)
    println!("1. Testing trade_fee_rate endpoint (all pairs)...");
    match account.trade_fee_rate(None) {
        Ok(response) => {
            let display = if response.len() > 500 {
                format!("{}... (truncated)", &response[..500])
            } else {
                response.clone()
            };
            println!("✓ Trade Fee Rate Response: {}\n", display);
        },
        Err(e) => println!("✗ Trade Fee Rate Error: {}\n", e),
    }

    // Test trade fee rate (specific pair)
    println!("2. Testing trade_fee_rate endpoint (lbk_usdt)...");
    match account.trade_fee_rate(Some("lbk_usdt")) {
        Ok(response) => println!("✓ Trade Fee Rate (LBK/USDT) Response: {}\n", response),
        Err(e) => println!("✗ Trade Fee Rate (LBK/USDT) Error: {}\n", e),
    }

    // Test API restrictions
    println!("3. Testing api_restrictions endpoint...");
    match account.api_restrictions() {
        Ok(response) => println!("✓ API Restrictions Response: {}\n", response),
        Err(e) => println!("✗ API Restrictions Error: {}\n", e),
    }

    // Test account info
    println!("4. Testing account_info endpoint...");
    match account.account_info() {
        Ok(response) => {
            let display = if response.len() > 500 {
                format!("{}... (truncated)", &response[..500])
            } else {
                response.clone()
            };
            println!("✓ Account Info Response: {}\n", display);
        },
        Err(e) => println!("✗ Account Info Error: {}\n", e),
    }

    println!("=== Completed Testing Synchronous Account Endpoints ===\n");
}

