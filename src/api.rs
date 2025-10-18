use crate::{
    client::{Client, AsyncClient},
    config::Config,
    common::{Common, AsyncCommon}
};

pub enum API {
    General(General),
    Market(Market),
    Wallet(Wallet),
    Spot(Spot),
    Account(Account),
    WebSocket(WebSocket),
}

pub enum General {
    CurrencyPairs,
    Accuracy,
    WithdrawConfigs,
    AssetConfigs,
    Timestamp,
}

pub enum Market {
    SystemPing,
    Depth,
    Price,
    BookTicker,
    Ticker24hr,
    EtfTicker24hr,
    Trades,
    Kline,
}

pub enum Wallet {
    SystemStatus,
    UserInfo,
    Withdraw,
    DepositHistory,
    WithdrawHistory,
    DepositAddress,
    AssetDetail,
}


pub enum Spot {
    OrderTest,
    CreateOrder,
    CancelOrder,
    CancelOrderBySymbol,
    OrderInfo,
    OpenOrders,
    OrderHistory,
    AccountInfo,
    TransactionHistory,
}

pub enum Account {
    TradeFeeRate,
    ApiRestrictions,
    AccountInfo,
}

pub enum WebSocket {
    // Public WebSocket endpoints
    KlineSubscribe,
    DepthSubscribe,
    TradeSubscribe,
    TickerSubscribe,
    // Private WebSocket endpoints
    GetSubscribeKey,
    RefreshSubscribeKey,
    DestroySubscribeKey,
    OrderUpdateSubscribe,
    AssetUpdateSubscribe,
}

// TODO: LBank Contract/Futures API - These endpoints need to be verified against LBank's actual Contract API
// The endpoints below are Binance-style and are NOT correct for LBank
/*
pub enum Futures {
    Ping,
    Time,
    ExchangeInfo,
    Depth,
    Trades,
    HistoricalTrades,
    AggTrades,
    Klines,
    ContinuousKlines,
    IndexPriceKlines,
    MarkPriceKlines,
    PremiumIndex,
    FundingRate,
    Ticker24hr,
    TickerPrice,
    BookTicker,
    AllForceOrders,
    AllOpenOrders,
    AllOrders,
    UserTrades,
    Order,
    PositionRisk,
    Balance,
    PositionSide,
    OpenInterest,
    OpenInterestHist,
    TopLongShortAccountRatio,
    TopLongShortPositionRatio,
    GlobalLongShortAccountRatio,
    TakerlongshortRatio,
    LvtKlines,
    IndexInfo,
    ChangeInitialLeverage,
    MarginType,
    PositionMargin,
    Account,
    OpenOrders,
    UserDataStream,
    Income,
}
*/

impl From<API> for String {
    fn from(item: API) -> Self {
        String::from(match item {
            API::General(route) => match route {
                General::CurrencyPairs => "/v2/currencyPairs.do",
                General::Accuracy => "/v2/accuracy.do",
                General::WithdrawConfigs => "/v2/withdrawConfigs.do",
                General::AssetConfigs => "/v2/assetConfigs.do",
                General::Timestamp => "/v2/timestamp.do",
            },
            API::Market(route) => match route {
                Market::SystemPing => "/v2/supplement/system_ping.do",
                Market::Depth => "/v2/depth.do",
                Market::Price => "/v2/supplement/ticker/price.do",
                Market::BookTicker => "/v2/supplement/ticker/bookTicker.do",
                Market::Ticker24hr => "/v2/ticker/24hr.do",
                Market::EtfTicker24hr => "/v2/etfTicker/24hr.do",
                Market::Trades => "/v2/supplement/trades.do",
                Market::Kline => "/v2/kline.do",
            },
            API::Wallet(route) => match route {
                Wallet::SystemStatus => "/v2/supplement/system_status.do",
                Wallet::UserInfo => "/v2/supplement/user_info.do",
                Wallet::Withdraw => "/v2/supplement/withdraw.do",
                Wallet::DepositHistory => "/v2/supplement/deposit_history.do",
                Wallet::WithdrawHistory => "/v2/supplement/withdraws.do",
                Wallet::DepositAddress => "/v2/supplement/get_deposit_address.do",
                Wallet::AssetDetail => "/v2/supplement/asset_detail.do",
            },
            API::Spot(route) => match route {
                Spot::OrderTest => "/v2/supplement/create_order_test.do",
                Spot::CreateOrder => "/v2/supplement/create_order.do",
                Spot::CancelOrder => "/v2/supplement/cancel_order.do",
                Spot::CancelOrderBySymbol => "/v2/supplement/cancel_order_by_symbol.do",
                Spot::OrderInfo => "/v2/supplement/orders_info.do",
                Spot::OpenOrders => "/v2/supplement/orders_info_no_deal.do",
                Spot::OrderHistory => "/v2/supplement/orders_info_history.do",
                Spot::AccountInfo => "/v2/supplement/user_info_account.do",
                Spot::TransactionHistory => "/v2/supplement/transaction_history.do",
            },
            API::Account(route) => match route {
                Account::TradeFeeRate => "/v2/supplement/customer_trade_fee.do",
                Account::ApiRestrictions => "/v2/supplement/api_Restrictions.do",
                Account::AccountInfo => "/v2/supplement/user_info_account.do",
            },
            API::WebSocket(route) => match route {
                WebSocket::GetSubscribeKey => "/v2/subscribe/get_key.do",
                WebSocket::RefreshSubscribeKey => "/v2/subscribe/refresh_key.do",
                WebSocket::DestroySubscribeKey => "/v2/subscribe/destroy_key.do",
                // WebSocket subscription endpoints don't have HTTP endpoints
                _ => "",
            },
        })
    }
}


/// Trait for blocking LBank API clients
pub trait LBank {
    fn new(api_key: Option<String>, api_secret: Option<String>) -> Self;
    fn new_with_config(api_key: Option<String>, api_secret: Option<String>, config: &Config) -> Self;
    fn set_verbose(&mut self, verbose: bool);
}

impl LBank for Common {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> Common {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>,
        secret_key: Option<String>,
        config: &Config,
    ) -> Common {
        Common {
            client: Client::new_with_config(api_key, secret_key, config),
        }
    }

    fn set_verbose(&mut self, verbose: bool) {
        self.client.set_verbose(verbose);
    }
}


/// Trait for async LBank API clients
pub trait AsyncLBank {
    fn new(api_key: Option<String>, api_secret: Option<String>) -> Self;
    fn new_with_config(api_key: Option<String>, api_secret: Option<String>, config: &Config) -> Self;
    fn set_verbose(&mut self, verbose: bool);
}

impl AsyncLBank for AsyncCommon {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> AsyncCommon {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>,
        secret_key: Option<String>,
        config: &Config,
    ) -> AsyncCommon {
        AsyncCommon {
            client: AsyncClient::new_with_config(api_key, secret_key, config),
        }
    }

    fn set_verbose(&mut self, verbose: bool) {
        self.client.set_verbose(verbose);
    }
}



