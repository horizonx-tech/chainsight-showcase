#[derive(Clone, Debug, Default, candid :: CandidType, serde :: Deserialize, serde :: Serialize)]
pub struct Account {
    pub address: String,
}
use uniswap_snapshotter_mainnet_bindings::get_uniswap_snapshotter_mainnet;
pub async fn calculate(targets: Vec<String>) -> Account {
    let _result: Result<String, String> =
        get_uniswap_snapshotter_mainnet(targets.get(0).unwrap().clone()).await;
    todo!()
}
