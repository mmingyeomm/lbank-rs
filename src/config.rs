
// API endpoints
pub const SPOT_MAINNET: &str = "https://www.lbkex.net";
pub const SPOT_WS_MAINNET: &str = "wss://www.lbkex.net/ws/V2/";
pub const FUTURES_MAINNET: &str = "https://fapi.lbkex.net";
pub const FUTURES_WS_MAINNET: &str = "wss://fapi.lbkex.net/ws";


#[derive(Clone, Debug)]
pub struct Config {
    pub rest_api_endpoint: String,
    pub ws_endpoint: String,

    pub futures_rest_api_endpoint: String,
    pub futures_ws_endpoint: String,

    pub recv_window: u64,

}


impl Default for Config {
    fn default() -> Self {
        Self {
            rest_api_endpoint: SPOT_MAINNET.into(),
            ws_endpoint: SPOT_WS_MAINNET.into(),

            futures_rest_api_endpoint: FUTURES_MAINNET.into(),
            futures_ws_endpoint: FUTURES_WS_MAINNET.into(),

            recv_window: 5000,
        }
    }
}