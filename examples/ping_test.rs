use lbank_rs::general::General;
use lbank_rs::api::LBank;

fn main() {
    println!("Testing LBank API ping...\n");
    
    // Create a blocking client (no API keys needed for ping)
    let mut general: General = LBank::new(None, None);
    
    // Enable verbose mode to see the URL
    general.set_verbose(true);
    
    println!("Calling ping()...\n");
    match general.ping() {
        Ok(response) => {
            println!("\n✅ Success!");
            println!("Response: {}", response);
        }
        Err(e) => {
            println!("\n❌ Error: {:?}", e);
        }
    }
}

