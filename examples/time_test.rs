use lbank_rs::general::General;
use lbank_rs::api::LBank;

fn main() {
    println!("Testing LBank API time endpoint (synchronous)...\n");

    // Create a client (no API keys needed for time endpoint)
    let mut general: General = LBank::new(None, None);

    // Enable verbose mode to see the URL
    general.set_verbose(true);

    println!("Making 5 time requests sequentially...\n");

    for i in 1..=5 {
        println!("Request #{}", i);
        match general.time() {
            Ok(response) => {
                println!("✅ Request #{} Success!", i);
                println!("Response: {}", response);
            }
            Err(e) => {
                println!("❌ Request #{} Error: {:?}", i, e);
            }
        }
        println!("---");
    }

    println!("\n🎉 All 5 requests completed!");
}

