use lbank_rs::general::AsyncGeneral;
use lbank_rs::api::AsyncLBank;

#[tokio::main]
async fn main() {
    println!("Testing LBank API time endpoint (asynchronous)...\n");

    // Create an async client (no API keys needed for time endpoint)
    let mut general: AsyncGeneral = AsyncLBank::new(None, None);

    // Enable verbose mode to see the URL
    general.set_verbose(true);

    println!("Making 5 time requests concurrently...\n");

    // Create async tasks
    let mut handles = Vec::new();

    for i in 1..=5 {
        let general_clone = general.clone();
        let handle = tokio::spawn(async move {
            match general_clone.time().await {
                Ok(response) => {
                    println!("✅ Request #{} Success!", i);
                    println!("Response: {}", response);
                }
                Err(e) => {
                    println!("❌ Request #{} Error: {:?}", i, e);
                }
            }
        });
        handles.push(handle);
    }

    // Wait for all requests to complete
    for handle in handles {
        let _ = handle.await;
    }

    println!("\n🎉 All 5 concurrent requests completed!");
}

