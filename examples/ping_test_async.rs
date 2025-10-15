use lbank_rs::general::AsyncGeneral;
use lbank_rs::api::AsyncLBank;

#[tokio::main]
async fn main() {
    println!("Testing LBank API ping (async)...\n");

    // Create an async client (no API keys needed for ping)
    let mut general: AsyncGeneral = AsyncLBank::new(None, None);

    // Enable verbose mode to see the URL
    general.set_verbose(true);

    println!("Calling ping()...\n");
    match general.ping().await {
        Ok(response) => {
            println!("\n✅ Success!");
            println!("Response: {}", response);
        }
        Err(e) => {
            println!("\n❌ Error: {:?}", e);
        }
    }
}
