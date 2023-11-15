use lens_vol_ethusd_accessors::*;
use lens_vol_ethusd_bindings::chainlink_ethusd::Snapshot;
pub type LensValue = u128;
use primitive_types::U256;

pub async fn calculate(targets: Vec<String>) -> LensValue {
    let prices = get_chainlink_ethusd(targets.get(0usize).unwrap().clone(), 24)
        .await
        .unwrap();

    to_rvol(prices)
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
        // Uniswap V3
        // result.push(calc_exchange_rate(
        //     U256::from_dec_str(price.value.as_str()).unwrap(),
        // ))
        result.push(U256::from_dec_str(price.value.as_str()).unwrap())
    }
    result
}

// This is for UniswapV3
// fn calc_exchange_rate(sqrt_price_x96: U256) -> U256 {
//     (sqrt_price_x96 * sqrt_price_x96) / U256::from(2).pow(U256::from(192))
// }

fn calc_realized_volatility(exchange_rates: Vec<U256>) -> f64 {
    let mut squared_r: Vec<U256> = Vec::new();
    for i in 0..exchange_rates.len() - 1 {
        let r = exchange_rates.get(i + 1).unwrap() / exchange_rates.get(i).unwrap();
        squared_r.push(r * r);
    }
    let sum_of_squared_r: f64 = squared_r.iter().map(|x| x.as_u128() as f64).sum();
    (sum_of_squared_r / squared_r.len() as f64).sqrt() * exchange_rates.len() as f64
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_calcrealized_volatility() {
        let prices = vec![
            Box::new(Snapshot {
                value: "2078036038910000000000".to_string(),
                timestamp: 0,
            }),
            Box::new(Snapshot {
                value: "2068036038910000000000".to_string(),
                timestamp: 0,
            }),
            Box::new(Snapshot {
                value: "2058036038910000000000".to_string(),
                timestamp: 0,
            }),
            Box::new(Snapshot {
                value: "2068036038910000000000".to_string(),
                timestamp: 0,
            }),
            Box::new(Snapshot {
                value: "2078036038910000000000".to_string(),
                timestamp: 0,
            }),
        ];
        assert_eq!(to_rvol(prices), 3535533905932737536);
    }
}