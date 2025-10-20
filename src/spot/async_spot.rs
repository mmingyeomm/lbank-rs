use crate::client::AsyncClient;
use crate::api::{API, Spot as SpotAPI};
use crate::errors::Result;
use crate::util::build_signed_request;
use std::collections::BTreeMap;

/// Spot Trading API client for asynchronous operations
#[derive(Clone)]
pub struct AsyncSpot {
    pub client: AsyncClient,
}

impl AsyncSpot {
    /// Test order creation without actually placing it
    /// 
    /// # Parameters
    /// * `symbol` - Trading pair (e.g., "eth_btc")
    /// * `type_` - Order type: "buy", "sell", "buy_market", "sell_market", "buy_maker", "sell_maker", "buy_ioc", "sell_ioc", "buy_fok", "sell_fok"
    /// * `price` - Order price (required for limit orders)
    /// * `amount` - Order amount (required)
    /// * `custom_id` - Optional user-defined order ID
    /// * `window` - Optional order expiration time in milliseconds
    pub async fn create_order_test(
        &self,
        symbol: &str,
        type_: &str,
        price: Option<&str>,
        amount: Option<&str>,
        custom_id: Option<&str>,
        window: Option<u64>,
    ) -> Result<String> {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".to_string(), symbol.to_string());
        parameters.insert("type".to_string(), type_.to_string());

        if let Some(p) = price {
            parameters.insert("price".to_string(), p.to_string());
        }
        if let Some(a) = amount {
            parameters.insert("amount".to_string(), a.to_string());
        }
        if let Some(cid) = custom_id {
            parameters.insert("custom_id".to_string(), cid.to_string());
        }
        if let Some(w) = window {
            parameters.insert("window".to_string(), w.to_string());
        }

        let signed_request = build_signed_request(
            parameters,
            &self.client.api_key,
            &self.client.secret_key,
        )?;
        self.client.post(API::Spot(SpotAPI::OrderTest), Some(signed_request)).await
    }

    /// Create/place a new order
    /// 
    /// # Parameters
    /// * `symbol` - Trading pair (e.g., "eth_btc")
    /// * `type_` - Order type: "buy", "sell", "buy_market", "sell_market", "buy_maker", "sell_maker", "buy_ioc", "sell_ioc", "buy_fok", "sell_fok"
    /// * `price` - Order price (required for limit orders)
    /// * `amount` - Order amount (required)
    /// * `custom_id` - Optional user-defined order ID
    /// * `window` - Optional order expiration time in milliseconds
    pub async fn create_order(
        &self,
        symbol: &str,
        type_: &str,
        price: Option<&str>,
        amount: Option<&str>,
        custom_id: Option<&str>,
        window: Option<u64>,
    ) -> Result<String> {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".to_string(), symbol.to_string());
        parameters.insert("type".to_string(), type_.to_string());

        if let Some(p) = price {
            parameters.insert("price".to_string(), p.to_string());
        }
        if let Some(a) = amount {
            parameters.insert("amount".to_string(), a.to_string());
        }
        if let Some(cid) = custom_id {
            parameters.insert("custom_id".to_string(), cid.to_string());
        }
        if let Some(w) = window {
            parameters.insert("window".to_string(), w.to_string());
        }

        let signed_request = build_signed_request(
            parameters,
            &self.client.api_key,
            &self.client.secret_key,
        )?;
        self.client.post(API::Spot(SpotAPI::CreateOrder), Some(signed_request)).await
    }

    /// Cancel a specific order
    /// 
    /// # Parameters
    /// * `symbol` - Trading pair
    /// * `order_id` - Order ID (must provide either order_id or orig_client_order_id)
    /// * `orig_client_order_id` - User-defined order ID (must provide either order_id or orig_client_order_id)
    pub async fn cancel_order(
        &self,
        symbol: &str,
        order_id: Option<&str>,
        orig_client_order_id: Option<&str>,
    ) -> Result<String> {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".to_string(), symbol.to_string());

        if let Some(oid) = order_id {
            parameters.insert("orderId".to_string(), oid.to_string());
        }
        if let Some(cid) = orig_client_order_id {
            parameters.insert("origClientOrderId".to_string(), cid.to_string());
        }

        let signed_request = build_signed_request(
            parameters,
            &self.client.api_key,
            &self.client.secret_key,
        )?;
        self.client.post(API::Spot(SpotAPI::CancelOrder), Some(signed_request)).await
    }

    /// Cancel all pending orders for a specific trading pair
    /// 
    /// # Parameters
    /// * `symbol` - Trading pair
    pub async fn cancel_order_by_symbol(&self, symbol: &str) -> Result<String> {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".to_string(), symbol.to_string());

        let signed_request = build_signed_request(
            parameters,
            &self.client.api_key,
            &self.client.secret_key,
        )?;
        self.client.post(API::Spot(SpotAPI::CancelOrderBySymbol), Some(signed_request)).await
    }

    /// Query specific order details
    /// 
    /// # Parameters
    /// * `symbol` - Trading pair
    /// * `order_id` - Order ID (must provide either order_id or orig_client_order_id)
    /// * `orig_client_order_id` - User-defined order ID (must provide either order_id or orig_client_order_id)
    pub async fn order_info(
        &self,
        symbol: &str,
        order_id: Option<&str>,
        orig_client_order_id: Option<&str>,
    ) -> Result<String> {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".to_string(), symbol.to_string());

        if let Some(oid) = order_id {
            parameters.insert("orderId".to_string(), oid.to_string());
        }
        if let Some(cid) = orig_client_order_id {
            parameters.insert("origClientOrderId".to_string(), cid.to_string());
        }

        let signed_request = build_signed_request(
            parameters,
            &self.client.api_key,
            &self.client.secret_key,
        )?;
        self.client.post(API::Spot(SpotAPI::OrderInfo), Some(signed_request)).await
    }

    /// Get all current open/pending orders for a trading pair
    /// 
    /// # Parameters
    /// * `symbol` - Trading pair
    /// * `current_page` - Current page number
    /// * `page_length` - Number of items per page (1-200)
    pub async fn open_orders(
        &self,
        symbol: &str,
        current_page: u32,
        page_length: u32,
    ) -> Result<String> {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".to_string(), symbol.to_string());
        parameters.insert("current_page".to_string(), current_page.to_string());
        parameters.insert("page_length".to_string(), page_length.to_string());

        let signed_request = build_signed_request(
            parameters,
            &self.client.api_key,
            &self.client.secret_key,
        )?;
        self.client.post(API::Spot(SpotAPI::OpenOrders), Some(signed_request)).await
    }

    /// Get historical orders (default: last 24 hours)
    /// 
    /// # Parameters
    /// * `symbol` - Trading pair
    /// * `current_page` - Current page number
    /// * `page_length` - Number of items per page (1-200)
    /// * `status` - Optional order status filter
    pub async fn order_history(
        &self,
        symbol: &str,
        current_page: u32,
        page_length: u32,
        status: Option<&str>,
    ) -> Result<String> {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".to_string(), symbol.to_string());
        parameters.insert("current_page".to_string(), current_page.to_string());
        parameters.insert("page_length".to_string(), page_length.to_string());

        if let Some(s) = status {
            parameters.insert("status".to_string(), s.to_string());
        }

        let signed_request = build_signed_request(
            parameters,
            &self.client.api_key,
            &self.client.secret_key,
        )?;
        self.client.post(API::Spot(SpotAPI::OrderHistory), Some(signed_request)).await
    }

    /// Get account information including balances and permissions
    pub async fn account_info(&self) -> Result<String> {
        let parameters: BTreeMap<String, String> = BTreeMap::new();
        
        let signed_request = build_signed_request(
            parameters,
            &self.client.api_key,
            &self.client.secret_key,
        )?;
        self.client.post(API::Spot(SpotAPI::AccountInfo), Some(signed_request)).await
    }

    /// Get historical transaction/trade details
    /// 
    /// # Parameters
    /// * `symbol` - Trading pair (e.g., "eth_btc", "btc_usdt")
    /// * `start_time` - Optional start time (yyyy-MM-dd or yyyy-MM-dd HH:mm:ss, UTC+8)
    /// * `end_time` - Optional end time (max 2 day window from start_time)
    /// * `from_id` - Optional starting transaction ID
    /// * `limit` - Optional number of results (default 100, max 100)
    pub async fn transaction_history(
        &self,
        symbol: &str,
        start_time: Option<&str>,
        end_time: Option<&str>,
        from_id: Option<&str>,
        limit: Option<u32>,
    ) -> Result<String> {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".to_string(), symbol.to_string());

        if let Some(st) = start_time {
            parameters.insert("startTime".to_string(), st.to_string());
        }
        if let Some(et) = end_time {
            parameters.insert("endTime".to_string(), et.to_string());
        }
        if let Some(fid) = from_id {
            parameters.insert("fromId".to_string(), fid.to_string());
        }
        if let Some(l) = limit {
            parameters.insert("limit".to_string(), l.to_string());
        }

        let signed_request = build_signed_request(
            parameters,
            &self.client.api_key,
            &self.client.secret_key,
        )?;
        self.client.post(API::Spot(SpotAPI::TransactionHistory), Some(signed_request)).await
    }
}

