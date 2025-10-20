use crate::client::AsyncClient;
use crate::api::{API, Account as AccountAPI};
use crate::errors::Result;
use crate::util::build_signed_request;
use std::collections::BTreeMap;

/// Account API client for asynchronous operations
#[derive(Clone)]
pub struct AsyncAccount {
    pub client: AsyncClient,
}

impl AsyncAccount {
    /// Query user's trading fee rates (maker/taker commission) for trading pairs
    /// 
    /// # Parameters
    /// * `category` - Optional trading pair filter (e.g., "lbk_usdt")
    pub async fn trade_fee_rate(&self, category: Option<&str>) -> Result<String> {
        let mut parameters = BTreeMap::new();

        if let Some(cat) = category {
            parameters.insert("category".to_string(), cat.to_string());
        }

        let signed_request = build_signed_request(
            parameters,
            &self.client.api_key,
            &self.client.secret_key,
        )?;
        self.client.post(API::Account(AccountAPI::TradeFeeRate), Some(signed_request)).await
    }

    /// Query API Key permissions (trading, reading, withdrawal, IP restrictions)
    pub async fn api_restrictions(&self) -> Result<String> {
        let parameters: BTreeMap<String, String> = BTreeMap::new();
        
        let signed_request = build_signed_request(
            parameters,
            &self.client.api_key,
            &self.client.secret_key,
        )?;
        self.client.post(API::Account(AccountAPI::ApiRestrictions), Some(signed_request)).await
    }

    /// Get account information including balances and permissions
    /// (Alias for Spot::account_info for convenience)
    pub async fn account_info(&self) -> Result<String> {
        let parameters: BTreeMap<String, String> = BTreeMap::new();
        
        let signed_request = build_signed_request(
            parameters,
            &self.client.api_key,
            &self.client.secret_key,
        )?;
        self.client.post(API::Account(AccountAPI::AccountInfo), Some(signed_request)).await
    }
}
