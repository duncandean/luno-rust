use serde::Deserialize;
use strum_macros::{Display, EnumString};

#[derive(EnumString, Display)]
pub enum Currency {
    EUR,
    IDR,
    MYR,
    NGN,
    ZAR,
    ZMW,
    BTC,
    XBT,
    ETH,
    BCH,
    XRP,
    LTC,
}

#[derive(EnumString, Display, Debug, Deserialize)]
pub enum TradingPair {
    XBTEUR,
    XBTIDR,
    XBTMYR,
    XBTNGN,
    XBTSGD,
    XBTUGX,
    XBTZAR,
    XBTZMW,
    BCHXBT,
    ETHMYR,
    ETHNGN,
    ETHZAR,
    ETHXBT,
    XRPZAR,
    XRPMYR,
    XRPXBT,
    LTCZAR,
    LTCXBT,
}

#[derive(Debug, Deserialize)]
pub struct Ticker {
    pub ask: String,
    pub timestamp: u64,
    pub bid: String,
    pub rolling_24_hour_volume: String,
    pub last_trade: String,
    pub pair: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TickerList {
    pub tickers: Option<Vec<Ticker>>,
}

#[derive(Debug, Deserialize)]
pub struct Bid {
    pub volume: String,
    pub price: String,
}

#[derive(Debug, Deserialize)]
pub struct Ask {
    pub volume: String,
    pub price: String,
}

#[derive(Debug, Deserialize)]
pub struct Orderbook {
    pub timestamp: u64,
    pub bids: Option<Vec<Bid>>,
    pub asks: Option<Vec<Ask>>,
}

#[derive(Debug, Deserialize)]
pub struct Trade {
    pub volume: String,
    pub timestamp: u64,
    pub price: String,
    pub is_buy: bool,
}

#[derive(Debug, Deserialize)]
pub struct TradeList {
    pub trades: Option<Vec<Trade>>,
}
