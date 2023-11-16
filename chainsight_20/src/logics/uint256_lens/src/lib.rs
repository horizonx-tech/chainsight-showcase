use chainsight_cdk::core::U256;
use ic_web3_rs::types::U256 as U256_1;
use uint256_lens_accessors::*;
const DECIMALS: u32 = 6;
pub type LensValue = U256;
pub async fn calculate(targets: Vec<String>) -> LensValue {
    let _result =
        get_get_last_snapshot_value_in_index_indexer(targets.get(0usize).unwrap().clone()).await;
    let idx = _result.unwrap().index;
    let v = idx * (10_u128.pow(DECIMALS) as f64);
    U256::from(U256_1::from(v.round() as u128))
}
