use candid::Principal;
use chainsight_cdk::rpc::{CallProvider, Caller, Message};
use ic_web3_rs::{
    ethabi::{encode, Token},
    types::U256,
};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Clone, Debug, Default, candid :: CandidType, serde :: Deserialize, serde :: Serialize)]
pub struct LensValue {
    pub value: Option<Vec<u8>>,
}
#[derive(Clone, Debug, Default, candid :: CandidType, serde :: Deserialize, serde :: Serialize)]
pub struct CalculateArgs {
    source_canister_id: String,
    method_name: String,
    args: HashMap<String, String>,
    plugins: Vec<Plugin>,
}
#[derive(Clone, Debug, Default, candid :: CandidType, serde :: Deserialize, serde :: Serialize)]
pub struct Plugin {
    canister_id: String,
    method_name: String,
    args: HashMap<String, String>,
}

pub async fn calculate(_targets: Vec<String>, args: CalculateArgs) -> LensValue {
    let caller = CallProvider::new();

    let mut value = handle_call(
        &caller,
        args.source_canister_id,
        &args.method_name,
        args.args,
    )
    .await;
    if value.is_none() {
        return LensValue { value: None };
    }

    for plugin in args.plugins {
        value = handle_call(
            &caller,
            plugin.canister_id,
            &plugin.method_name,
            plugin.args,
        )
        .await;
        if value.is_none() {
            return LensValue { value: None };
        }
    }
    
    let token_value = U256::from_dec_str(&value.unwrap().value().to_string());
    return LensValue {
        value: Some(encode(&[Token::Uint(token_value.unwrap())])),
    };
}

async fn handle_call<T>(
    caller: &CallProvider,
    canister_id: String,
    method_name: &str,
    args: T,
) -> Option<chainsight_cdk::core::U256>
where
    T: Serialize,
{
    let target = Principal::from_text(canister_id.clone());
    if target.is_err() {
        return None;
    }

    let message = Message::new::<T>(args, target.unwrap(), method_name);
    if message.is_err() {
        return None;
    }

    let call_result = caller.call(message.unwrap()).await;
    if call_result.is_err() {
        return None;
    }

    let response = call_result.unwrap().reply::<chainsight_cdk::core::U256>();
    if response.is_err() {
        return None;
    }

    return Some(response.unwrap());
}
