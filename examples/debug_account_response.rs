use lbank_rs::client::Client;
use lbank_rs::api::{API, Spot};
use lbank_rs::util::build_signed_request;
use std::collections::BTreeMap;

fn main() {
    println!("Debug: LBank Account API Response\n");

    let api_key = "cecf5cad-3c0f-405f-81eb-9bcd914ea08d".to_string();
    let secret_key = "FEF8E225E1D470D7B3E4D99A31BD7237".to_string();

    let mut client = Client::new(Some(api_key.clone()), Some(secret_key.clone()));
    client.set_verbose(true);

    println!("📊 Testing Account endpoint...\n");
    
    let parameters: BTreeMap<String, String> = BTreeMap::new();
    match build_signed_request(parameters, &api_key, &secret_key) {
        Ok(request) => {
            println!("✅ Signed request built: {}\n", request);
            
            match client.post(API::Spot(Spot::Account), Some(request)) {
                Ok(response) => {
                    println!("✅ Raw API Response:");
                    println!("{}", response);
                    println!("\n---\n");
                    
                    // Try to pretty print if it's JSON
                    if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&response) {
                        println!("📋 Pretty JSON:");
                        println!("{}", serde_json::to_string_pretty(&json_value).unwrap());
                    }
                }
                Err(e) => {
                    println!("❌ Error: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to build signed request: {:?}", e);
        }
    }
}

