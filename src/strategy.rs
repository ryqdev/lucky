
#[derive(Debug)]
pub struct Strategy{
    risk_rate: f32,
    expected_annual_return: f32
}

impl Strategy{
    pub fn new(r: f32, e: f32) -> Strategy {
        Strategy{ risk_rate: r, expected_annual_return: e }
    }
}
