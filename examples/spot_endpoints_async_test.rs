use lbank_rs::{spot::AsyncSpot, client::AsyncClient};
use dotenv::dotenv;
use std::env;
use std::sync::Arc;

/// Example for testing Async Spot Trading endpoints
/// 
/// NOTE: All spot endpoints require authentication with valid API credentials.
/// Create a .env file with your credentials or set environment variables:
/// 
/// .env file:
///   LBANK_API_KEY=your_api_key
///   LBANK_SECRET_KEY=your_secret_key
/// 
/// Usage:
///   cargo run --example spot_endpoints_async_test
/// 
/// WARNING: create_order() function performs real trades!
///          Use create_order_test() for safe testing.

#[tokio::main]
async fn main() {
    // Load .env file if it exists
    dotenv().ok();
    
    println!("\n=== Testing Asynchronous Spot Trading Endpoints ===\n");
    
    // Get API credentials from environment variables
    let api_key = env::var("LBANK_API_KEY").ok();
    let secret_key = env::var("LBANK_SECRET_KEY").ok();

    if api_key.is_none() || secret_key.is_none() {
        println!("⚠️  Error: No API credentials found in environment");
        println!("   Set LBANK_API_KEY and LBANK_SECRET_KEY to test spot endpoints\n");
        println!("   All spot endpoints require authentication.\n");
        return;
    }

    // Create async spot client
    let client = AsyncClient::new(api_key, secret_key);
    let spot: Arc<AsyncSpot> = Arc::new(AsyncSpot { client });

    // Test account info
    let spot1 = spot.clone();
    let handle1 = tokio::spawn(async move {
        println!("1. Testing account_info endpoint...");
        match spot1.account_info().await {
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

    // Test order test (safe - doesn't place real order)
    let spot2 = spot.clone();
    let handle2 = tokio::spawn(async move {
        println!("2. Testing create_order_test endpoint (LBK/USDT buy test)...");
        match spot2.create_order_test(
            "lbk_usdt",
            "buy",
            Some("0.01"),     // Price: 0.01 USDT
            Some("100"),      // Amount: 100 LBK
            None,             // No custom_id
            None,             // No window
        ).await {
            Ok(response) => println!("✓ Order Test Response: {}\n", response),
            Err(e) => println!("✗ Order Test Error: {}\n", e),
        }
    });

    // Test open orders
    let spot3 = spot.clone();
    let handle3 = tokio::spawn(async move {
        println!("3. Testing open_orders endpoint (LBK/USDT)...");
        match spot3.open_orders("lbk_usdt", 1, 10).await {
            Ok(response) => {
                let display = if response.len() > 500 {
                    format!("{}... (truncated)", &response[..500])
                } else {
                    response.clone()
                };
                println!("✓ Open Orders Response: {}\n", display);
            },
            Err(e) => println!("✗ Open Orders Error: {}\n", e),
        }
    });

    // Test order history
    let spot4 = spot.clone();
    let handle4 = tokio::spawn(async move {
        println!("4. Testing order_history endpoint (LBK/USDT)...");
        match spot4.order_history("lbk_usdt", 1, 10, None).await {
            Ok(response) => {
                let display = if response.len() > 500 {
                    format!("{}... (truncated)", &response[..500])
                } else {
                    response.clone()
                };
                println!("✓ Order History Response: {}\n", display);
            },
            Err(e) => println!("✗ Order History Error: {}\n", e),
        }
    });

    // Test transaction history
    let spot5 = spot.clone();
    let handle5 = tokio::spawn(async move {
        println!("5. Testing transaction_history endpoint (LBK/USDT)...");
        match spot5.transaction_history(
            "lbk_usdt",
            None,      // No start time (default: recent)
            None,      // No end time
            None,      // No from_id
            Some(10),  // Limit to 10 results
        ).await {
            Ok(response) => {
                let display = if response.len() > 500 {
                    format!("{}... (truncated)", &response[..500])
                } else {
                    response.clone()
                };
                println!("✓ Transaction History Response: {}\n", display);
            },
            Err(e) => println!("✗ Transaction History Error: {}\n", e),
        }
    });

    // Join all handles
    let _ = tokio::try_join!(
        handle1,
        handle2,
        handle3,
        handle4,
        handle5,
    ).expect("Failed to join handles");

    println!("=== Completed Testing Asynchronous Spot Trading Endpoints ===\n");
    println!("Note: The following endpoints were NOT tested for safety:");
    println!("      - create_order() - Places real orders");
    println!("      - cancel_order() - Cancels real orders");
    println!("      - cancel_order_by_symbol() - Cancels all orders");
    println!("      - order_info() - Requires an existing order ID");
    println!("\n      Use these carefully in production with proper validation!\n");
}

