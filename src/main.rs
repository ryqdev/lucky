mod common;
mod account;
mod strategy;
mod strategy_engine;
mod operation_engine;

use strategy::Strategy;
use strategy_engine::StrategyEngine;
use account::Account;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let account = Account::new(520000.0, 10.0);
    let strategy = Strategy::new(5.0, 5.0);
    let mut engine = StrategyEngine::new(strategy, account);
    engine.run()?;
    Ok(())
}
