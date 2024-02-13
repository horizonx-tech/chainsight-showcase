use weighted_average_price_lens_accessors::*;
#[derive(Clone, Debug, Default, candid :: CandidType, serde :: Deserialize, serde :: Serialize)]
pub struct LensValue {
    pub res_chainlink: String,
    pub res_api3: String,
    pub res_web: String,
}
pub async fn calculate(targets: Vec<String>) -> LensValue {
    let res_chainlink =
        get_chainlink_eth_indexer(targets.get(0usize).unwrap().clone())
            .await.unwrap();
    let res_api3 =
        get_api3_eth_indexer(targets.get(1usize).unwrap().clone()).await.unwrap();
    let res_web =
        get_web_eth_indexer(targets.get(2usize).unwrap().clone()).await.unwrap();
    
    LensValue {
        res_chainlink: format!("{:?}", res_chainlink),
        res_api3: format!("{:?}", res_api3),
        res_web: format!("{:?}", res_web),
    }
}
