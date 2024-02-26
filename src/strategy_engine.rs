use std::io::Read;
use error_chain::error_chain;
use serde::Deserialize;
use crate::operation_engine::OperationEngine;
use crate::strategy::Strategy;


error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[derive(Debug)]
pub(crate) struct Order{
    id: String,
    timestamp: i64,
    bid_ask: crate::common::BidAsk,
    pub(crate) volume: f32,
    symbol: String
}

pub(crate) struct MarketData {
    symbol: String,
    timestamp: i64,
    pub(crate) price: f32,
    order_book: OrderBook,
}

struct OrderBook {
    bid_volume_list:Vec<i32>,
    ask_volume_list:Vec<i32>,
    price_list:Vec<i32>
}



#[derive(Deserialize)]
struct FetchPrice {
    symbol: String,
    price: String
}



pub struct StrategyEngine {
    strategy: Strategy,
    market_data_api: String
}


impl StrategyEngine {
    pub fn new(s: Strategy) -> StrategyEngine {
        StrategyEngine{
            strategy: s,
            market_data_api: crate::common::BINANCE_PRICE_API.to_string()
        }
    }

    pub fn run(&self){
        log::info!("StrategyEngine is running, the strategy is {:?}", self.strategy);
        loop{
            let market_data = self.fetch_market_data();
            if market_data.price < crate::common::LOWER_BOUND_PRICE {
                self.send_order(Order {
                    id: "test_bid".to_string(),
                    timestamp: 0,
                    bid_ask: crate::common::BidAsk::BID,
                    volume: 1.0,
                    symbol: crate::common::BTC_USDT.to_string(),
                })
            } else if market_data.price > crate::common::UPPER_BOUND_PRICE {
                self.send_order(Order {
                    id: "test_ask".to_string(),
                    timestamp: 0,
                    bid_ask: crate::common::BidAsk::ASK,
                    volume: 1.0,
                    symbol: crate::common::BTC_USDT.to_string(),
                })
            }
        }
    }

    fn fetch_market_data(&self) -> MarketData {
        let body = self.fetch_price().unwrap_or_else(|error| {
            log::error!("fetch price failed, error: {}", error);
            String::from(crate::common::ERROR_FETCH_DATA)
        });

        if body != crate::common::ERROR_FETCH_DATA {
            log::info!("Fetch: {}", body);
        }

        let price = self.fetch_price_from_json(&body).unwrap_or_else(|error| {
            log::error!("parse json failed, error: {}", error);
            crate::common::ERROR_PRICE
        } );

        if price != crate::common::ERROR_PRICE {
            log::info!("Price: {}", price);
        }

        MarketData {
            symbol: crate::common::BTC_USDT.to_string(),
            timestamp: 0,
            price,
            order_book: OrderBook {
                bid_volume_list: vec![],
                ask_volume_list: vec![],
                price_list: vec![],
            },
        }
    }

    fn fetch_price_from_json(&self, body: &str) -> serde_json::Result<(f32)> {
        let fp: FetchPrice = serde_json::from_str(body)?;
        Ok((fp.price).parse().unwrap())
    }

    fn fetch_price(&self) -> Result<String> {
        let mut res = reqwest::blocking::get("https://api.binance.com/api/v3/ticker/price?symbol=BTCUSDT")?;
        let mut body = String::new();
        res.read_to_string(&mut body)?;

        Ok(body)
    }

    fn send_order(&self, order: Order){
        //TODO: new an OperationEngine whenever send order? Might be not suitable
        let op = OperationEngine::new();
        if order.bid_ask == crate::common::BidAsk::BID {
            op.bid(order)
        } else if order.bid_ask == crate::common::BidAsk::ASK {
            op.ask(order)
        } else {
            log::error!("Unknown order type")
        }
    }
}
