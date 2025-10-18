use crate::client::Client;
use crate::api::{API, Wallet as WalletAPI};
use crate::errors::Result;
use crate::util::build_signed_request;
use std::collections::BTreeMap;

/// Wallet API client for synchronous operations
#[derive(Clone)]
pub struct Wallet {
    pub client: Client,
}

impl Wallet {
    /// Get system status (0: maintenance, 1: normal)
    /// Note: This endpoint doesn't require authentication according to docs
    pub fn system_status(&self) -> Result<String> {
        self.client.post(API::Wallet(WalletAPI::SystemStatus), None)
    }

    /// Get all coin balances and multi-chain information for user
    /// Requires authentication
    pub fn user_info(&self) -> Result<String> {
        let parameters: BTreeMap<String, String> = BTreeMap::new();
        let signed_request = build_signed_request(
            parameters,
            &self.client.api_key,
            &self.client.secret_key,
        )?;
        self.client.post(API::Wallet(WalletAPI::UserInfo), Some(signed_request))
    }

    /// Submit withdrawal request (supports multi-chain)
    /// 
    /// # Parameters
    /// * `address` - Withdrawal address (or transfer account if type=1)
    /// * `coin` - Currency code
    /// * `amount` - Withdrawal amount
    /// * `fee` - Withdrawal fee
    /// * `network_name` - Optional chain name
    /// * `memo` - Optional memo (for BTS, DCT, etc.)
    /// * `mark` - Optional withdrawal notes
    /// * `name` - Optional address remark (adds to address book)
    /// * `withdraw_order_id` - Optional custom withdrawal ID
    /// * `type_` - Optional type (1 for intra-site transfer)
    pub fn withdraw(
        &self,
        address: &str,
        coin: &str,
        amount: &str,
        fee: &str,
        network_name: Option<&str>,
        memo: Option<&str>,
        mark: Option<&str>,
        name: Option<&str>,
        withdraw_order_id: Option<&str>,
        type_: Option<&str>,
    ) -> Result<String> {
        let mut parameters = BTreeMap::new();
        parameters.insert("address".to_string(), address.to_string());
        parameters.insert("coin".to_string(), coin.to_string());
        parameters.insert("amount".to_string(), amount.to_string());
        parameters.insert("fee".to_string(), fee.to_string());

        if let Some(network) = network_name {
            parameters.insert("networkName".to_string(), network.to_string());
        }
        if let Some(m) = memo {
            parameters.insert("memo".to_string(), m.to_string());
        }
        if let Some(mk) = mark {
            parameters.insert("mark".to_string(), mk.to_string());
        }
        if let Some(n) = name {
            parameters.insert("name".to_string(), n.to_string());
        }
        if let Some(wid) = withdraw_order_id {
            parameters.insert("withdrawOrderId".to_string(), wid.to_string());
        }
        if let Some(t) = type_ {
            parameters.insert("type".to_string(), t.to_string());
        }

        let signed_request = build_signed_request(
            parameters,
            &self.client.api_key,
            &self.client.secret_key,
        )?;
        self.client.post(API::Wallet(WalletAPI::Withdraw), Some(signed_request))
    }

    /// Get deposit history with optional filters
    /// 
    /// # Parameters
    /// * `status` - Optional deposit status filter: "1" (Applying), "2" (Success), "3" (Failed), "4" (Cancelled), "5" (Transfer)
    /// * `coin` - Optional currency filter
    /// * `start_time` - Optional start time (timestamp in milliseconds)
    /// * `end_time` - Optional end time (timestamp in milliseconds)
    pub fn deposit_history(
        &self,
        status: Option<&str>,
        coin: Option<&str>,
        start_time: Option<u64>,
        end_time: Option<u64>,
    ) -> Result<String> {
        let mut parameters = BTreeMap::new();

        if let Some(s) = status {
            parameters.insert("status".to_string(), s.to_string());
        }
        if let Some(c) = coin {
            parameters.insert("coin".to_string(), c.to_string());
        }
        if let Some(st) = start_time {
            parameters.insert("startTime".to_string(), st.to_string());
        }
        if let Some(et) = end_time {
            parameters.insert("endTime".to_string(), et.to_string());
        }

        let signed_request = build_signed_request(
            parameters,
            &self.client.api_key,
            &self.client.secret_key,
        )?;
        self.client.post(API::Wallet(WalletAPI::DepositHistory), Some(signed_request))
    }

    /// Get withdrawal history with optional filters
    /// 
    /// # Parameters
    /// * `status` - Optional status: "1" (Applying), "2" (Cancelled), "3" (Failed), "4" (Completed)
    /// * `coin` - Optional currency filter
    /// * `withdraw_order_id` - Optional custom withdrawal ID filter
    /// * `start_time` - Optional start time (timestamp in milliseconds)
    /// * `end_time` - Optional end time (timestamp in milliseconds)
    pub fn withdraw_history(
        &self,
        status: Option<&str>,
        coin: Option<&str>,
        withdraw_order_id: Option<&str>,
        start_time: Option<u64>,
        end_time: Option<u64>,
    ) -> Result<String> {
        let mut parameters = BTreeMap::new();

        if let Some(s) = status {
            parameters.insert("status".to_string(), s.to_string());
        }
        if let Some(c) = coin {
            parameters.insert("coin".to_string(), c.to_string());
        }
        if let Some(wid) = withdraw_order_id {
            parameters.insert("withdrawOrderId".to_string(), wid.to_string());
        }
        if let Some(st) = start_time {
            parameters.insert("startTime".to_string(), st.to_string());
        }
        if let Some(et) = end_time {
            parameters.insert("endTime".to_string(), et.to_string());
        }

        let signed_request = build_signed_request(
            parameters,
            &self.client.api_key,
            &self.client.secret_key,
        )?;
        self.client.post(API::Wallet(WalletAPI::WithdrawHistory), Some(signed_request))
    }

    /// Get deposit address for a specific coin and chain
    /// 
    /// # Parameters
    /// * `coin` - Currency code
    /// * `network_name` - Optional chain name
    pub fn deposit_address(&self, coin: &str, network_name: Option<&str>) -> Result<String> {
        let mut parameters = BTreeMap::new();
        parameters.insert("coin".to_string(), coin.to_string());

        if let Some(network) = network_name {
            parameters.insert("networkName".to_string(), network.to_string());
        }

        let signed_request = build_signed_request(
            parameters,
            &self.client.api_key,
            &self.client.secret_key,
        )?;
        self.client.post(API::Wallet(WalletAPI::DepositAddress), Some(signed_request))
    }

    /// Get asset details including withdrawal/deposit status and fees
    /// 
    /// # Parameters
    /// * `coin` - Optional currency filter
    pub fn asset_detail(&self, coin: Option<&str>) -> Result<String> {
        let mut parameters = BTreeMap::new();

        if let Some(c) = coin {
            parameters.insert("coin".to_string(), c.to_string());
        }

        let signed_request = build_signed_request(
            parameters,
            &self.client.api_key,
            &self.client.secret_key,
        )?;
        self.client.post(API::Wallet(WalletAPI::AssetDetail), Some(signed_request))
    }
}

