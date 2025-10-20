use lbank_rs::{account::AsyncAccount, client::AsyncClient};
use dotenv::dotenv;
use std::env;
use std::sync::Arc;

/// Example for testing Async Account endpoints
/// 
/// NOTE: All account endpoints require authentication with valid API credentials.
/// Create a .env file with your credentials or set environment variables:
/// 
/// .env file:
///   LBANK_API_KEY=your_api_key
///   LBANK_SECRET_KEY=your_secret_key
/// 
/// Usage:
///   cargo run --example account_endpoints_async_test

#[tokio::main]
async fn main() {
    // Load .env file if it exists
    dotenv().ok();
    
    println!("\n=== Testing Asynchronous Account Endpoints ===\n");
    
    // Get API credentials from environment variables
    let api_key = env::var("LBANK_API_KEY").ok();
    let secret_key = env::var("LBANK_SECRET_KEY").ok();

    if api_key.is_none() || secret_key.is_none() {
        println!("⚠️  Error: No API credentials found in environment");
        println!("   Set LBANK_API_KEY and LBANK_SECRET_KEY to test account endpoints\n");
        println!("   All account endpoints require authentication.\n");
        return;
    }

    // Create async account client
    let client = AsyncClient::new(api_key, secret_key);
    let account: Arc<AsyncAccount> = Arc::new(AsyncAccount { client });

    // Test trade fee rate (all pairs)
    let account1 = account.clone();
    let handle1 = tokio::spawn(async move {
        println!("1. Testing trade_fee_rate endpoint (all pairs)...");
        match account1.trade_fee_rate(None).await {
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
    });

    // Test trade fee rate (specific pair)
    let account2 = account.clone();
    let handle2 = tokio::spawn(async move {
        println!("2. Testing trade_fee_rate endpoint (lbk_usdt)...");
        match account2.trade_fee_rate(Some("lbk_usdt")).await {
            Ok(response) => println!("✓ Trade Fee Rate (LBK/USDT) Response: {}\n", response),
            Err(e) => println!("✗ Trade Fee Rate (LBK/USDT) Error: {}\n", e),
        }
    });

    // Test API restrictions
    let account3 = account.clone();
    let handle3 = tokio::spawn(async move {
        println!("3. Testing api_restrictions endpoint...");
        match account3.api_restrictions().await {
            Ok(response) => println!("✓ API Restrictions Response: {}\n", response),
            Err(e) => println!("✗ API Restrictions Error: {}\n", e),
        }
    });

    // Test account info
    let account4 = account.clone();
    let handle4 = tokio::spawn(async move {
        println!("4. Testing account_info endpoint...");
        match account4.account_info().await {
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
    });

    // Join all handles
    let _ = tokio::try_join!(
        handle1,
        handle2,
        handle3,
        handle4,
    ).expect("Failed to join handles");

    println!("=== Completed Testing Asynchronous Account Endpoints ===\n");
}

