use log;

pub(crate) const BTC_USDT: &str = "BTC_USDT";
pub(crate) const BINANCE_PRICE_API: &str = "https://api.binance.com/api/v3/ticker/price?symbol=BTCUSDT";
pub(crate) const BINANCE_ORDER_API: &str = "";

pub(crate) const ERROR_FETCH_DATA: &str = "Error fetching price";
pub(crate) const ERROR_PRICE: f32 = -1.0;

pub(crate) const LOWER_BOUND_PRICE: f32 = 50000.0;
pub(crate) const UPPER_BOUND_PRICE: f32 = 52000.0;

#[derive(PartialEq, Debug)]
pub(crate) enum BidAsk{
    BID,
    ASK
}

