use std::collections::HashMap;

use crate::config::{DataSource, Destination, RelayConfig, RelayItem};
use candid::Principal;
use chainsight_cdk::rpc::{CallProvider, Caller, Message};
#[derive(Clone, Debug, Default, candid :: CandidType, serde :: Deserialize, serde :: Serialize)]
struct EncodeRequest {
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

type EncodeResponse = Option<Vec<u8>>;

async fn index() {
    let config = get_relay_config();
    let issuer = ic_cdk::id();

    let fetcher = CallProvider::new();
    let mut values = Vec::<Vec<u8>>::new();
    let mut keys = Vec::<[u8; 32usize]>::new();

    for item in config.items {
        let content = EncodeRequest {
            source_canister_id: item.source.canister_id.to_string(),
            method_name: item.source.method_name.clone(),
            args: item.source.args.clone(),
            plugins: item.plugins.clone(),
        };
        let message = Message::new::<EncodeRequest>(
            content,
            item.source.canister_id.clone(),
            &item.source.method_name,
        );
        if message.is_err() {
            // TODO log error
            continue;
        }

        let call_result = fetcher.call(message.unwrap()).await;
        if call_result.is_err() {
            // TODO log error
            continue;
        }

        let response = call_result.unwrap().reply::<EncodeResponse>();
        if response.is_err() {
            // TODO log error
            continue;
        }

        let value = response.unwrap();
        if value.is_none() {
            continue;
        }

        values.push(value.unwrap());
        keys.push(item.key(issuer));
    }

    update_state_bulk(values, keys)
}

fn key(item: RelayItem) -> [u8; 32usize] {
    item.key(ic_cdk::id())
}

fn add_relay_item(item: RelayItem) -> [u8; 32usize] {
    if ic_cdk::api::is_controller(&ic_cdk::caller()) {
        panic!("Not authorized")
    }
    let mut config = get_relay_config();
    config.items.push(item.clone());

    item.key(ic_cdk::id())
}

fn delete_relay_item(key: [u8; 32usize]) {
    if ic_cdk::api::is_controller(&ic_cdk::caller()) {
        panic!("Not authorized")
    }
    let issuer = ic_cdk::id();
    let mut config = get_relay_config();
    config.items.retain(|item| item.key(issuer) != key);
}

fn update_relay_item(item: RelayItem) {
    if ic_cdk::api::is_controller(&ic_cdk::caller()) {
        panic!("Not authorized")
    }
    let issuer = ic_cdk::id();
    let mut config = get_relay_config();
    for e in config.items.iter_mut() {
        if e.key(issuer) == item.key(issuer) {
            *e = item.clone();
            return;
        }
    }
    panic!("Item not found")
}

/// mock
fn get_relay_config() -> RelayConfig {
    let mut config = RelayConfig::default();

    config.destination = Destination {
        address: "".to_string(),
        rpc_url: "".to_string(),
        chain_id: 1,
    };

    let item = RelayItem {
        requester_id: "requester".to_string(),
        encoder: "uint256".to_string(),
        source: DataSource {
            canister_id: Principal::from_text("filter").unwrap(),
            method_name: "encode".to_string(),
            args: HashMap::new(),
        },
        plugins: Vec::new(),
    };
    config.items.push(item);

    config
}
fn update_state_bulk(values: Vec<Vec<u8>>, keys: Vec<[u8; 32usize]>) {
    // TODO call update state bulk
}
