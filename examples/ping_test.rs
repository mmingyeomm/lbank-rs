use lbank_rs::common::Common;
use lbank_rs::api::LBank;

fn main() {
    println!("Testing LBank API ping (10 requests - blocking)...\n");

    // Create a client (no API keys needed for ping)
    let mut common: Common = LBank::new(None, None);

    // Enable verbose mode to see the URL
    common.set_verbose(true);

    println!("Making 10 ping requests sequentially...\n");

    for i in 1..=30 {
        println!("Request #{}", i);
        match common.ping() {
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

    println!("\n🎉 All 30 requests completed!");
}
