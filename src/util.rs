use crate::errors::Result;
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use md5::{Md5, Digest};

/// Build a signed request for LBank API
pub fn build_signed_request(
    mut parameters: BTreeMap<String, String>,
    api_key: &str,
    secret_key: &str,
) -> Result<String> {
    // Add timestamp if not present
    if !parameters.contains_key("timestamp") {
        let timestamp = get_timestamp();
        parameters.insert("timestamp".into(), timestamp.to_string());
    }
    
    // Add api_key
    parameters.insert("api_key".into(), api_key.into());
    
    // Add signature_method (required by LBank)
    parameters.insert("signature_method".into(), "RSA".into());
    
    // Add echostr (random string required by LBank)
    if !parameters.contains_key("echostr") {
        let echostr = uuid_spot();
        parameters.insert("echostr".into(), echostr);
    }
    
    // Build the signature string
    let signature = sign_request(&parameters, secret_key)?;
    parameters.insert("sign".into(), signature);
    
    // Build query string
    let query_string = build_query_string(&parameters);
    Ok(query_string)
}

/// Sign the request parameters using MD5 (for LBank API)
fn sign_request(parameters: &BTreeMap<String, String>, secret_key: &str) -> Result<String> {
    // Sort parameters and create sign string (LBank format)
    let mut sign_str = String::new();
    for (key, value) in parameters.iter() {
        if key != "sign" {
            sign_str.push_str(key);
            sign_str.push('=');
            sign_str.push_str(value);
            sign_str.push('&');
        }
    }
    
    // Remove trailing &
    if sign_str.ends_with('&') {
        sign_str.pop();
    }
    
    // Add secret_key at the end (as per LBank documentation)
    sign_str.push_str(&format!("&secret_key={}", secret_key));
    
    // Calculate MD5 hash
    let mut hasher = Md5::new();
    hasher.update(sign_str.as_bytes());
    let result = hasher.finalize();
    
    // Convert to uppercase hex string
    let signature = format!("{:X}", result);
    
    Ok(signature)
}

/// Build query string from parameters
pub fn build_query_string(parameters: &BTreeMap<String, String>) -> String {
    parameters
        .iter()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect::<Vec<String>>()
        .join("&")
}

/// Get current timestamp in milliseconds
pub fn get_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as u64
}

/// Check if start time is valid (less than current time)
pub fn is_start_time_valid(start_time: &u64) -> bool {
    let current_time = get_timestamp();
    start_time < &current_time
}

/// Generate a UUID for spot orders
pub fn uuid_spot() -> String {
    Uuid::new_v4().to_string().replace("-", "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_timestamp() {
        let ts = get_timestamp();
        assert!(ts > 0);
    }

    #[test]
    fn test_is_start_time_valid() {
        let past_time = get_timestamp() - 10000;
        assert!(is_start_time_valid(&past_time));
        
        let future_time = get_timestamp() + 10000;
        assert!(!is_start_time_valid(&future_time));
    }

    #[test]
    fn test_uuid_spot() {
        let uuid = uuid_spot();
        assert_eq!(uuid.len(), 32); // UUID without dashes is 32 chars
        assert!(!uuid.contains('-'));
    }

    #[test]
    fn test_build_query_string() {
        let mut params = BTreeMap::new();
        params.insert("symbol".to_string(), "eth_btc".to_string());
        params.insert("type".to_string(), "buy".to_string());
        
        let query = build_query_string(&params);
        assert!(query.contains("symbol=eth_btc"));
        assert!(query.contains("type=buy"));
    }
}

