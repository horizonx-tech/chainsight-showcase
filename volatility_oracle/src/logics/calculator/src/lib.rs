use calculator_accessors::*;
pub type LensValue = u128;
pub async fn calculate(targets: Vec<String>) -> LensValue {
    let prices = get_prices(targets.get(0).unwrap().to_string(), 24).await;
    to_rvol(prices)
}

async fn get_prices(target: String, size: u64) -> Vec<Box<Snapshot>> {
    let result = get_uniswap_snapshotter_mainnet(target, size).await.unwrap();
    result
}

fn to_rvol(prices: Vec<Box<Snapshot>>) -> u128 {
    let exchange_rates = calc_exchange_rates(prices);
    let rvol = calc_realized_volatility(exchange_rates);
    (rvol * 1000000000000000000.0) as u128
}

fn calc_exchange_rates(prices: Vec<Box<Snapshot>>) -> Vec<U256> {
    let mut result: Vec<U256> = Vec::new();
    for i in 0..prices.len() {
        let price = prices.get(i).unwrap();
        result.push(calc_exchange_rate(
            U256::from_dec_str(price.as_ref().value.0.as_str()).unwrap(),
        ))
    }
    result
}

fn calc_exchange_rate(sqrt_price_x96: U256) -> U256 {
    (sqrt_price_x96 * sqrt_price_x96) / U256::from(2).pow(U256::from(192))
}

fn calc_realized_volatility(exchange_rates: Vec<U256>) -> f64 {
    let mut squared_r: Vec<U256> = Vec::new();
    for i in 0..exchange_rates.len() - 1 {
        let r = exchange_rates.get(i + 1).unwrap() / exchange_rates.get(i).unwrap();
        squared_r.push(r * r);
    }
    let sum_of_squared_r: f64 = squared_r.iter().map(|x| x.as_u128() as f64).sum();
    (sum_of_squared_r / squared_r.len() as f64).sqrt() * exchange_rates.len() as f64
}

use calculator_bindings::uniswap_snapshotter_mainnet::Snapshot;
use ic_web3_rs::types::U256;

async fn _get_target_proxy(target: candid::Principal) -> candid::Principal {
    let out: ic_cdk::api::call::CallResult<(candid::Principal,)> =
        ic_cdk::api::call::call(target, "get_proxy", ()).await;
    out.unwrap().0
}
