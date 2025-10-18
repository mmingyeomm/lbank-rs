use lbank_rs::{api::LBank, common::Common};

fn main() {
    println!("\n=== Testing Synchronous General Endpoints ===\n");
    
    // Create a new Common instance without authentication (public endpoints)
    let common: Common = LBank::new(None, None);

    // Test currency pairs endpoint
    println!("Testing currency_pairs endpoint...");
    match common.currency_pairs() {
        Ok(response) => println!("✓ Currency Pairs Response: {}\n", response),
        Err(e) => println!("✗ Currency Pairs Error: {}\n", e),
    }

    // Test accuracy endpoint
    println!("Testing accuracy endpoint...");
    match common.accuracy() {
        Ok(response) => println!("✓ Accuracy Response: {}\n", response),
        Err(e) => println!("✗ Accuracy Error: {}\n", e),
    }

    // Test withdraw configs endpoint
    println!("Testing withdraw_configs endpoint...");
    match common.withdraw_configs() {
        Ok(response) => println!("✓ Withdraw Configs Response: {}\n", response),
        Err(e) => println!("✗ Withdraw Configs Error: {}\n", e),
    }

    // Test asset configs endpoint
    println!("Testing asset_configs endpoint...");
    match common.asset_configs() {
        Ok(response) => println!("✓ Asset Configs Response: {}\n", response),
        Err(e) => println!("✗ Asset Configs Error: {}\n", e),
    }

    // Test timestamp endpoint
    println!("Testing time endpoint...");
    match common.time() {
        Ok(response) => println!("✓ Server Time Response: {}\n", response),
        Err(e) => println!("✗ Server Time Error: {}\n", e),
    }

    println!("=== Completed Testing Synchronous General Endpoints ===\n");
}
