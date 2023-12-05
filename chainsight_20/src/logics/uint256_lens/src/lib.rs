use uint256_lens_accessors::*;
const DECIMALS: u32 = 6;
pub type LensValue = u128;
pub async fn calculate(targets: Vec<String>) -> LensValue {
    let _result =
        get_get_last_snapshot_value_in_index_indexer(targets.get(0usize).unwrap().clone()).await;
    let idx = _result.unwrap().index;
    let v = idx * (10_u128.pow(DECIMALS) as f64);
    v.round() as u128
}
