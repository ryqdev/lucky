mod common;
mod account;
mod strategy;
mod strategy_engine;
mod operation_engine;

use strategy::Strategy;
use strategy_engine::StrategyEngine;
use account::Account;
use rust_decimal_macros::dec;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let account = Account::new(dec!(520000.0), dec!(10.0));
    let strategy = Strategy::new(5.0, 5.0);
    let mut engine = StrategyEngine::new(strategy, account);
    engine.run()?;
    Ok(())
}
