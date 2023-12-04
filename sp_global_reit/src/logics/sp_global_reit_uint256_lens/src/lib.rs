use sp_global_reit_uint256_lens_accessors::*;
pub type LensValue = u128;
const DECIMALS: u32 = 8;
pub async fn calculate(targets: Vec<String>) -> LensValue {
    let result = get_get_last_snapshot_value_in_sp_global_reit_indexer_https(
        targets.get(0usize).unwrap().clone(),
    )
    .await
    .unwrap();
    let market_price = result.chart.result[0].meta.regularMarketPrice;
    convert_to_u128(market_price)
}
fn convert_to_u128(value: f64) -> LensValue {
    let value = value * (10u128.pow(DECIMALS) as f64);
    value as LensValue
}
