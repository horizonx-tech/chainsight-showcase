pub type LensValue  = u128;

const PRECISION: u32 = 18;
pub async fn calculate(targets: Vec<String>) -> LensValue {
    let result = get_chainlink_ethusd(targets.get(0).unwrap().clone())
        .await
        .unwrap();
    let raw = result.parse::<u128>().unwrap();
    // raw data is 8 digits and convert it into 18 digits
    raw * 10u128.pow(PRECISION - 8)
}
algorithm_lens_finder!("chainlink_ethusd", "proxy_get_last_snapshot_value", String);
use chainsight_cdk::lens::LensFinder;
use chainsight_cdk_macros::algorithm_lens_finder;
async fn _get_target_proxy(target: candid::Principal) -> candid::Principal {
    let out: ic_cdk::api::call::CallResult<(candid::Principal,)> =
        ic_cdk::api::call::call(target, "get_proxy", ()).await;
    out.unwrap().0
}
