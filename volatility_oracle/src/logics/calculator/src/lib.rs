#[derive(Clone, Debug, Default, candid :: CandidType, serde :: Deserialize, serde :: Serialize)]
pub struct Price {
    pub observation_index: u16,
    pub sqrt_price_x96: u128,
    pub block_timestamp: u64,
}
use uniswap_snapshotter_mainnet_bindings::{
    get_uniswap_snapshotter_mainnet, get_uniswap_snapshotter_mainnet_unwrap,
};
pub async fn calculate(targets: Vec<String>) -> Price {
    let mainnet = get_uniswap_snapshotter_mainnet(targets.get(0).unwrap().clone())
        .await
        .unwrap();
    todo!();
}
