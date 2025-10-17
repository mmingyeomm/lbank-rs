use lbank_rs::Account::AsyncAccount;
use lbank_rs::client::AsyncClient;

#[tokio::main]
async fn main() {
    println!("Testing LBank Account - Get Balance (Asynchronous)\n");

    // Hardcoded API credentials - REPLACE WITH YOUR ACTUAL KEYS
    let api_key = "cecf5cad-3c0f-405f-81eb-9bcd914ea08d".to_string();
    let secret_key = "FEF8E225E1D470D7B3E4D99A31BD7237".to_string();

    println!("✅ Using hardcoded API credentials");
    println!("API Key: {}...\n", &api_key[..std::cmp::min(10, api_key.len())]);

    // Create async account client
    let client = AsyncClient::new(Some(api_key), Some(secret_key));
    let account = AsyncAccount { client };

    // Test 1: Get full account information
    println!("📊 Test 1: Getting full account information...");
    match account.get_account().await {
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

    // Test 2: Get multiple balances concurrently
    println!("📊 Test 2: Getting multiple balances concurrently...");
    
    let btc_future = account.get_balance("btc");
    let eth_future = account.get_balance("eth");
    let usdt_future = account.get_balance("usdt");

    let results = tokio::join!(btc_future, eth_future, usdt_future);

    match results.0 {
        Ok(balance) => println!("✅ BTC - Free: {}, Locked: {}", balance.free, balance.locked),
        Err(e) => println!("❌ BTC error: {:?}", e),
    }

    match results.1 {
        Ok(balance) => println!("✅ ETH - Free: {}, Locked: {}", balance.free, balance.locked),
        Err(e) => println!("❌ ETH error: {:?}", e),
    }

    match results.2 {
        Ok(balance) => println!("✅ USDT - Free: {}, Locked: {}", balance.free, balance.locked),
        Err(e) => println!("❌ USDT error: {:?}", e),
    }

    println!("\n---\n");

    // Test 3: Try to get balance for non-existent asset
    println!("📊 Test 3: Getting balance for non-existent asset...");
    match account.get_balance("NONEXISTENT").await {
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

