use std::os::macos::raw::time_t;

fn main() {
}

const BTC_USDT: &str = "BTC_USDT";
const ETH_USDT: &str = "ETH_USDT";

enum BidAsk{
    BID,
    ASK
}

trait Strategy {
    // fn new() -> Self
    //     where
    //         Self: Sized;

    fn fetch_market_data() -> MarketData{
        MarketData{
            symbol: BTC_USDT.to_string(),
            timestamp: 0,
            price: 0.0,
            order_book: OrderBook {
                bid_volume_list: vec![],
                ask_volume_list: vec![],
                price_list: vec![],
            },
        }
    }

    fn send_order() -> Order {
        Order{
            id: "order".to_string(),
            timestamp: 0,
            bid_ask: BidAsk::BID,
            volume: 0,
            symbol: BTC_USDT,
        }
    }
}

trait Operation {
    // fn new() -> Self
    //     where
    //         Self: Sized;
    fn bid() {

    }
    fn ask() {

    }
    fn isfilled() -> bool {
        return true
    }
}

struct Order{
    id: String,
    timestamp: time_t,
    bid_ask: BidAsk,
    volume: i32,
    symbol: *str
}

struct MarketData {
    symbol: String,
    timestamp: time_t,
    price: f32,
    order_book: OrderBook,
}

struct OrderBook {
    bid_volume_list:Vec<i32>,
    ask_volume_list:Vec<i32>,
    price_list:Vec<i32>
}

