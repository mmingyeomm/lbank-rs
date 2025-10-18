use dotenv::dotenv;
use std::env;
use base64::{Engine as _, engine::general_purpose};

fn main() {
    dotenv().ok();

    let secret_key = env::var("LBANK_SECRET_KEY").expect("LBANK_SECRET_KEY must be set");

    println!("Secret key length: {}", secret_key.len());
    println!("First 50 chars: {}", &secret_key[..secret_key.len().min(50)]);
    println!("Contains 'BEGIN': {}", secret_key.contains("BEGIN"));
    println!("Contains newlines: {}", secret_key.contains('\n'));

    // Try to decode as base64
    match general_purpose::STANDARD.decode(&secret_key) {
        Ok(decoded) => {
            println!("✅ Successfully decoded as base64, length: {}", decoded.len());
            println!("First 20 bytes: {:?}", &decoded[..decoded.len().min(20)]);
        }
        Err(e) => {
            println!("❌ Failed to decode as base64: {}", e);
        }
    }
}
