use gold_algorithm_lens_accessors::*;
pub type LensValue = u128;
const DECIMALS: u32 = 8;
pub async fn calculate(targets: Vec<String>) -> LensValue {
    let result = get_get_last_snapshot_value_in_gold_snapshot_indexer_https(
        targets.get(0usize).unwrap().clone(),
    )
    .await;
    let market_price = result.unwrap().chart.result[0].meta.regularMarketPrice;
    convert_to_u128(market_price)
}

fn convert_to_u128(value: f64) -> u128 {
    let v = value * (10u128.pow(DECIMALS) as f64);
    v.round() as u128
}
