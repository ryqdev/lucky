use std::io::Read;
use std::{thread, time};
use serde::Deserialize;
use crate::account::Account;
use crate::operation_engine::OperationEngine;
use crate::strategy::Strategy;
use crate::common::BidAsk;

#[derive(Debug)]
pub struct Order{
    id: String,
    timestamp: i64,
    pub(crate) bid_ask: BidAsk,
    pub volume: f32,
    pub price: f32,
    symbol: String
}

pub struct MarketData {
    symbol: String,
    timestamp: i64,
    pub price: f32,
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
    account: Account,
    market_data_api: String
}

impl StrategyEngine {
    pub fn new(s: Strategy, a: Account) -> StrategyEngine {
        StrategyEngine{
            strategy: s,
            account: a,
            market_data_api: crate::common::BINANCE_PRICE_API.to_string()
        }
    }

    pub fn run(&mut self) -> anyhow::Result<()>{
        log::info!("StrategyEngine is running, the strategy is {:?}", self.strategy);
        let mut old_price = 50000.0;
        loop{
            let market_data = self.fetch_market_data()?;
            log::info!("Balance is {}", self.account.balance);
            log::info!("Total Assets is {}", self.account.balance + self.account.position * market_data.price);
            log::info!("old price is {}, current price is {}", old_price, market_data.price);
            if (old_price - market_data.price) / old_price > crate::common::THRESHOLD_RATE{
                self.send_order(Order {
                    id: "test_bid".to_string(),
                    timestamp: 0,
                    bid_ask: crate::common::BidAsk::BID,
                    volume: 1.0,
                    price: market_data.price,
                    symbol: crate::common::BTC_USDT.to_string(),
                })
            } else if (market_data.price - old_price) / old_price > crate::common::THRESHOLD_RATE {
                self.send_order(Order {
                    id: "test_ask".to_string(),
                    timestamp: 0,
                    bid_ask: crate::common::BidAsk::ASK,
                    volume: 1.0,
                    price: market_data.price,
                    symbol: crate::common::BTC_USDT.to_string(),
                })
            }
            old_price = market_data.price;
            thread::sleep(time::Duration::from_secs(60));
        }
    }

    fn fetch_market_data(&self) -> anyhow::Result<MarketData> {
        let body = self.fetch_price()?;

        let price = self.fetch_price_from_json(&body)?;

        Ok(MarketData {
            symbol: crate::common::BTC_USDT.to_string(),
            timestamp: 0,
            price,
            order_book: OrderBook {
                bid_volume_list: vec![],
                ask_volume_list: vec![],
                price_list: vec![],
            },
        })
    }

    fn fetch_price_from_json(&self, body: &str) -> serde_json::Result<(f32)> {
        let fp: FetchPrice = serde_json::from_str(body)?;
        Ok((fp.price).parse().unwrap())
    }

    fn fetch_price(&self) -> anyhow::Result<String> {
        let mut res = reqwest::blocking::get("https://api.binance.com/api/v3/ticker/price?symbol=BTCUSDT")?;
        let mut body = String::new();
        res.read_to_string(&mut body)?;

        Ok(body)
    }

    fn send_order(&mut self, order: Order){
        //TODO: new an OperationEngine whenever send order? Might be not suitable
        let op = OperationEngine::new();
        if order.bid_ask == crate::common::BidAsk::BID {
            op.bid(order, &mut self.account)
        } else if order.bid_ask == crate::common::BidAsk::ASK {
            op.ask(order, &mut self.account)
        } else {
            log::error!("Unknown order type")
        }
    }
}
