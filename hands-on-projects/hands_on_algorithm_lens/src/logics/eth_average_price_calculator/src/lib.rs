use eth_average_price_calculator_accessors::*;
use std::str::FromStr;
#[derive(Clone, Debug, Default, candid :: CandidType, serde :: Deserialize, serde :: Serialize)]
pub struct LensValue {
    pub value: f64,
}
pub async fn calculate(targets: Vec<String>) -> LensValue {
    let result = get_chainlink_eth_indexer(targets.get(0usize).unwrap().clone()).await;
    let price_from_chainlink = result.unwrap().1;

    let result = get_api3_eth_indexer(targets.get(1usize).unwrap().clone()).await;
    let price_from_api3 = result.unwrap().0;

    let result = get_web_eth_indexer(targets.get(2usize).unwrap().clone()).await;
    let price_from_web = result.unwrap().ethereum.usd;

    let prices = vec![
        f64::from_str(&price_from_chainlink).unwrap() / 10f64.powi(8),
        f64::from_str(&price_from_api3).unwrap() / 10f64.powi(18),
        price_from_web,
    ];
    ic_cdk::println!("{:?}", &prices);
    let average = prices.iter().sum::<f64>() / prices.len() as f64;

    LensValue { value: average }
}

