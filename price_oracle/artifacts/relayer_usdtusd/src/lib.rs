use chainsight_cdk::rpc::{CallProvider, Caller, Message};
use chainsight_cdk_macros::{
    chainsight_common, define_get_ethereum_address, define_transform_for_web3, define_web3_ctx,
    did_export, init_in, manage_single_state, relayer_source, setup_func, timer_task_func,
};
use ic_web3_rs::types::{Address, U256};
use std::str::FromStr;
chainsight_common!(3600);
define_web3_ctx!();
define_transform_for_web3!();
manage_single_state!("target_addr", String, false);
manage_single_state!("target_canister", String, false);
setup_func ! ({ target_canister : String , target_addr : String , web3_ctx_param : chainsight_cdk :: web3 :: Web3CtxParam });
define_get_ethereum_address!();
timer_task_func!("set_task", "sync", true);
init_in!();
ic_solidity_bindgen::contract_abi!("./__interfaces/Uint128Oracle.json");
relayer_source!(true);
pub type CallCanisterResponse = u128;
mod app;
type CallCanisterArgs = Vec<String>;
pub fn call_args() -> Vec<String> {
    vec![
        "br5f7-7uaaa-aaaaa-qaaca-cai".to_string(),
        "b77ix-eeaaa-aaaaa-qaada-cai".to_string(),
        "by6od-j4aaa-aaaaa-qaadq-cai".to_string(),
    ]
}
async fn sync() {
    let target_canister = candid::Principal::from_text(get_target_canister()).unwrap();
    let call_result = CallProvider::new()
        .call(
            Message::new::<CallCanisterArgs>(
                call_args(),
                _get_target_proxy(target_canister.clone()).await,
                "proxy_get_result",
            )
            .unwrap(),
        )
        .await;
    if let Err(err) = call_result {
        ic_cdk::println!("error: {:?}", err);
        return;
    }
    let val = call_result.unwrap().reply::<CallCanisterResponse>();
    if let Err(err) = val {
        ic_cdk::println!("error: {:?}", err);
        return;
    }
    let datum = val.unwrap();
    if !app::filter(&datum) {
        return;
    }
    Uint128Oracle::new(
        Address::from_str(&get_target_addr()).unwrap(),
        &web3_ctx().unwrap(),
    )
    .update_state(datum as u128, None)
    .await
    .unwrap();
    ic_cdk::println!("value_to_sync={:?}", datum);
}
did_export!("relayer_usdtusd");
