use std::sync::Arc;
use lbank_rs::{api::AsyncLBank, common::AsyncCommon};

#[tokio::main]
async fn main() {
    println!("\n=== Testing Asynchronous General Endpoints ===\n");
    
    // Create a new AsyncCommon instance without authentication (public endpoints)
    let async_common: Arc<AsyncCommon> = Arc::new(AsyncLBank::new(None, None));

    // Create handle for currency pairs endpoint
    let client1 = async_common.clone();
    let handle1 = tokio::spawn(async move {
        println!("Testing currency_pairs endpoint...");
        match client1.currency_pairs().await {
            Ok(response) => println!("✓ Currency Pairs Response: {}\n", response),
            Err(e) => println!("✗ Currency Pairs Error: {}\n", e),
        }
    });

    // Create handle for accuracy endpoint
    let client2 = async_common.clone();
    let handle2 = tokio::spawn(async move {
        println!("Testing accuracy endpoint...");
        match client2.accuracy().await {
            Ok(response) => println!("✓ Accuracy Response: {}\n", response),
            Err(e) => println!("✗ Accuracy Error: {}\n", e),
        }
    });

    // Create handle for withdraw configs endpoint
    let client3 = async_common.clone();
    let handle3 = tokio::spawn(async move {
        println!("Testing withdraw_configs endpoint...");
        match client3.withdraw_configs().await {
            Ok(response) => println!("✓ Withdraw Configs Response: {}\n", response),
            Err(e) => println!("✗ Withdraw Configs Error: {}\n", e),
        }
    });

    // Create handle for asset configs endpoint
    let client4 = async_common.clone();
    let handle4 = tokio::spawn(async move {
        println!("Testing asset_configs endpoint...");
        match client4.asset_configs().await {
            Ok(response) => println!("✓ Asset Configs Response: {}\n", response),
            Err(e) => println!("✗ Asset Configs Error: {}\n", e),
        }
    });

    // Create handle for timestamp endpoint
    let client5 = async_common.clone();
    let handle5 = tokio::spawn(async move {
        println!("Testing time endpoint...");
        match client5.time().await {
            Ok(response) => println!("✓ Server Time Response: {}\n", response),
            Err(e) => println!("✗ Server Time Error: {}\n", e),
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

    println!("=== Completed Testing Asynchronous General Endpoints ===\n");
}