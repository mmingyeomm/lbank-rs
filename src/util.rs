use crate::errors::Result;
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use md5::{Md5, Digest};
use rsa::{RsaPrivateKey, pkcs8::{DecodePrivateKey, LineEnding}, pkcs1::DecodeRsaPrivateKey, Pkcs1v15Sign};
use sha2::Sha256;
use base64::{Engine as _, engine::general_purpose};
use hmac::{Hmac, Mac};

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

    // Detect signature method based on secret key format
    // If secret key is short (< 100 chars) and looks like hex, use HmacSHA256
    // Otherwise use RSA
    let signature_method = if secret_key.len() < 100 && secret_key.chars().all(|c| c.is_ascii_hexdigit()) {
        "HmacSHA256"
    } else {
        "RSA"
    };

    // Add signature_method (required by LBank)
    parameters.insert("signature_method".into(), signature_method.into());

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

/// Sign the request parameters using either RSA or HmacSHA256 (for LBank API)
///
/// According to LBank API documentation:
/// For both RSA and HmacSHA256:
/// 1. Sort parameters and create parameter string
/// 2. Calculate MD5 hash of parameter string (uppercase hex)
///
/// For RSA:
/// 3. Sign the MD5 hash using RSA private key with SHA256
/// 4. Base64 encode the signature
///
/// For HmacSHA256:
/// 3. HMAC-SHA256 hash the MD5 string using secret key
/// 4. Convert to uppercase hex
fn sign_request(parameters: &BTreeMap<String, String>, secret_key: &str) -> Result<String> {
    // Step 1: Sort parameters and create sign string (LBank format)
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

    // Step 2: Calculate MD5 hash and convert to uppercase hex
    let mut hasher = Md5::new();
    hasher.update(sign_str.as_bytes());
    let result = hasher.finalize();
    let md5_hex = format!("{:X}", result);

    // Check signature method from parameters
    let signature_method = parameters.get("signature_method")
        .map(|s| s.as_str())
        .unwrap_or("RSA");

    match signature_method {
        "HmacSHA256" => {
            // HmacSHA256 method
            type HmacSha256 = Hmac<sha2::Sha256>;
            let mut mac = HmacSha256::new_from_slice(secret_key.as_bytes())
                .map_err(|e| format!("Invalid HMAC key: {}", e))?;
            mac.update(md5_hex.as_bytes());
            let result = mac.finalize();
            let code_bytes = result.into_bytes();

            // Convert to hex string (lowercase per API example)
            let signature = hex::encode(code_bytes);
            Ok(signature)
        }
        "RSA" => {
            // RSA method
            let private_key = if secret_key.contains("BEGIN") {
                // It's in PEM format
                RsaPrivateKey::from_pkcs8_pem(secret_key)
                    .or_else(|_| RsaPrivateKey::from_pkcs1_pem(secret_key))
                    .map_err(|e| format!("Failed to parse RSA private key from PEM: {}", e))?
            } else {
                // Try as base64-encoded DER
                let key_der = general_purpose::STANDARD
                    .decode(secret_key)
                    .map_err(|e| format!("Failed to decode secret key from base64: {}", e))?;

                RsaPrivateKey::from_pkcs8_der(&key_der)
                    .or_else(|_| RsaPrivateKey::from_pkcs1_der(&key_der))
                    .map_err(|e| format!("Failed to parse RSA private key from DER: {}", e))?
            };

            // Hash the MD5 hex string with SHA256
            let mut sha256_hasher = Sha256::new();
            sha256_hasher.update(md5_hex.as_bytes());
            let hashed = sha256_hasher.finalize();

            // Sign using RSA with PKCS#1 v1.5 padding
            let signature = private_key
                .sign(Pkcs1v15Sign::new::<Sha256>(), &hashed)
                .map_err(|e| format!("Failed to sign: {}", e))?;

            // Base64 encode the signature
            let signature_base64 = general_purpose::STANDARD.encode(&signature);
            Ok(signature_base64)
        }
        _ => Err(format!("Unsupported signature method: {}", signature_method).into())
    }
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

