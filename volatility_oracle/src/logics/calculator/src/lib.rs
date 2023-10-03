#[derive(Clone, Debug, Default, candid :: CandidType, serde :: Deserialize, serde :: Serialize)]
pub struct LensValue {
    pub dummy: u64,
}
pub async fn calculate(targets: Vec<String>) -> LensValue {
    let _result = get_uniswap_snapshotter_mainnet(targets.get(0).unwrap().clone(), 0).await;
    let _result = get_uniswap_snapshotter_polygon_mumbai(targets.get(0).unwrap().clone(), 0).await;
    todo!()
}

use uniswap_snapshotter_mainnet_bindings::Snapshot as Snapshot_uniswap_snapshotter_mainnet;
algorithm_lens_finder!(
    "uniswap_snapshotter_mainnet",
    "proxy_get_top_snapshots",
    Vec<Snapshot_uniswap_snapshotter_mainnet>,
    u64
);
use uniswap_snapshotter_polygon_mumbai_bindings::Snapshot as Snapshot_uniswap_snapshotter_polygon_mumbai;
algorithm_lens_finder!(
    "uniswap_snapshotter_polygon_mumbai",
    "proxy_get_top_snapshots",
    Vec<Snapshot_uniswap_snapshotter_polygon_mumbai>,
    u64
);
use chainsight_cdk::lens::LensFinder;
use chainsight_cdk_macros::algorithm_lens_finder;
async fn _get_target_proxy(target: candid::Principal) -> candid::Principal {
    let out: ic_cdk::api::call::CallResult<(candid::Principal,)> =
        ic_cdk::api::call::call(target, "get_proxy", ()).await;
    out.unwrap().0
}
