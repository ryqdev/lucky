mod common;
mod account;
mod strategy;
mod strategy_engine;
mod operation_engine;

use strategy::Strategy;
use strategy_engine::StrategyEngine;
use account::Account;

fn main() {
    env_logger::init();
    let account = Account::new(100.0, 100.0);
    let strategy = Strategy::new(5.0, 5.0);
    let engine = StrategyEngine::new(strategy);
    engine.run();
}
