use lbank_rs::{wallet::AsyncWallet, client::AsyncClient};
use dotenv::dotenv;
use std::env;
use std::sync::Arc;

/// Example for testing Async Wallet endpoints
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
///   cargo run --example wallet_endpoints_async_test
/// 
/// WARNING: Be careful with withdraw() function - it performs real withdrawals!

#[tokio::main]
async fn main() {
    // Load .env file if it exists
    dotenv().ok();
    
    println!("\n=== Testing Asynchronous Wallet Endpoints ===\n");
    
    // Get API credentials from environment variables
    let api_key = env::var("LBANK_API_KEY").ok();
    let secret_key = env::var("LBANK_SECRET_KEY").ok();

    // Create async wallet client
    let client = if api_key.is_some() && secret_key.is_some() {
        AsyncClient::new(api_key.clone(), secret_key.clone())
    } else {
        println!("⚠️  Warning: No API credentials found in environment");
        println!("   Set LBANK_API_KEY and LBANK_SECRET_KEY to test authenticated endpoints\n");
        AsyncClient::new(None, None)
    };

    let wallet: Arc<AsyncWallet> = Arc::new(AsyncWallet { client });

    // Test system status (public endpoint - no auth required)
    let wallet1 = wallet.clone();
    let handle1 = tokio::spawn(async move {
        println!("1. Testing system_status endpoint...");
        match wallet1.system_status().await {
            Ok(response) => println!("✓ System Status Response: {}\n", response),
            Err(e) => println!("✗ System Status Error: {}\n", e),
        }
    });

    // Wait for system status to complete
    let _ = handle1.await;

    // Only test authenticated endpoints if credentials are provided
    if api_key.is_none() || secret_key.is_none() {
        println!("Skipping authenticated endpoints (no credentials provided)\n");
        println!("=== Completed Testing Async Wallet Endpoints ===\n");
        return;
    }

    // Get time range for history queries
    let end_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    let start_time = end_time - (30 * 24 * 60 * 60 * 1000); // 30 days ago

    // Test user info
    let wallet2 = wallet.clone();
    let handle2 = tokio::spawn(async move {
        println!("2. Testing user_info endpoint...");
        match wallet2.user_info().await {
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
    });

    // Test deposit history
    let wallet3 = wallet.clone();
    let handle3 = tokio::spawn(async move {
        println!("3. Testing deposit_history endpoint (USDT, last 30 days)...");
        match wallet3.deposit_history(None, Some("usdt"), Some(start_time), Some(end_time)).await {
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
    });

    // Test withdrawal history
    let wallet4 = wallet.clone();
    let handle4 = tokio::spawn(async move {
        println!("4. Testing withdraw_history endpoint (last 30 days)...");
        match wallet4.withdraw_history(None, None, None, Some(start_time), Some(end_time)).await {
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
    });

    // Test deposit address
    let wallet5 = wallet.clone();
    let handle5 = tokio::spawn(async move {
        println!("5. Testing deposit_address endpoint (USDT)...");
        match wallet5.deposit_address("usdt", None).await {
            Ok(response) => println!("✓ Deposit Address Response: {}\n", response),
            Err(e) => println!("✗ Deposit Address Error: {}\n", e),
        }
    });

    // Test asset detail (single coin)
    let wallet6 = wallet.clone();
    let handle6 = tokio::spawn(async move {
        println!("6. Testing asset_detail endpoint (USDT)...");
        match wallet6.asset_detail(Some("usdt")).await {
            Ok(response) => println!("✓ Asset Detail Response: {}\n", response),
            Err(e) => println!("✗ Asset Detail Error: {}\n", e),
        }
    });

    // Test asset detail (all coins)
    let wallet7 = wallet.clone();
    let handle7 = tokio::spawn(async move {
        println!("7. Testing asset_detail endpoint (all coins)...");
        match wallet7.asset_detail(None).await {
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
    });

    // Join all handles
    let _ = tokio::try_join!(
        handle2,
        handle3,
        handle4,
        handle5,
        handle6,
        handle7,
    ).expect("Failed to join handles");

    println!("=== Completed Testing Asynchronous Wallet Endpoints ===\n");
    println!("Note: withdraw() endpoint not tested for safety reasons");
    println!("      Use it carefully in production with proper validation!\n");
}

