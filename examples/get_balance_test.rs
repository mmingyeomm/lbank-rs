use lbank_rs::Account::Account;
use lbank_rs::client::Client;

fn main() {
    println!("Testing LBank Account - Get Balance (Synchronous)\n");

    // Hardcoded API credentials - REPLACE WITH YOUR ACTUAL KEYS
    let api_key = "cecf5cad-3c0f-405f-81eb-9bcd914ea08d".to_string();
    let secret_key = "FEF8E225E1D470D7B3E4D99A31BD7237".to_string();
    
    println!("✅ Using hardcoded API credentials");
    println!("API Key: {}...\n", &api_key[..std::cmp::min(10, api_key.len())]);

    // Create account client
    let mut client = Client::new(Some(api_key), Some(secret_key));
    client.set_verbose(true);  // Enable verbose mode to see requests
    let account = Account { client };

    // Test 1: Get full account information
    println!("📊 Test 1: Getting full account information...");
    match account.get_account() {
        Ok(account_info) => {
            println!("✅ Account info retrieved successfully!");
            println!("Result: {}", account_info.result);
            println!("Error Code: {}", account_info.error_code);
            println!("Number of assets: {}", account_info.data.balances.len());
            println!("\n📋 All Balances:");
            for balance in &account_info.data.balances {
                println!("  {} - Free: {}, Locked: {}", 
                    balance.asset, balance.free, balance.locked);
            }
        }
        Err(e) => {
            println!("❌ Failed to get account info: {:?}", e);
            return;
        }
    }

    println!("\n---\n");

    // Test 2: Get specific asset balance (BTC)
    println!("📊 Test 2: Getting BTC balance...");
    match account.get_balance("btc") {
        Ok(balance) => {
            println!("✅ BTC balance retrieved!");
            println!("Asset: {}", balance.asset);
            println!("Free: {}", balance.free);
            println!("Locked: {}", balance.locked);
        }
        Err(e) => {
            println!("❌ Failed to get BTC balance: {:?}", e);
        }
    }

    println!("\n---\n");

    // Test 3: Get USDT balance
    println!("📊 Test 3: Getting USDT balance...");
    match account.get_balance("usdt") {
        Ok(balance) => {
            println!("✅ USDT balance retrieved!");
            println!("Asset: {}", balance.asset);
            println!("Free: {}", balance.free);
            println!("Locked: {}", balance.locked);
        }
        Err(e) => {
            println!("❌ Failed to get USDT balance: {:?}", e);
        }
    }

    println!("\n---\n");

    // Test 4: Try to get balance for non-existent asset
    println!("📊 Test 4: Getting balance for non-existent asset...");
    match account.get_balance("NONEXISTENT") {
        Ok(balance) => {
            println!("✅ Balance retrieved: {} - Free: {}, Locked: {}", 
                balance.asset, balance.free, balance.locked);
        }
        Err(e) => {
            println!("❌ Expected error: {:?}", e);
        }
    }

    println!("\n🎉 Balance tests completed!");
}

