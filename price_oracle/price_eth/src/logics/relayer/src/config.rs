use candid::{CandidType, Decode, Encode, Principal};
use ic_stable_structures::Storable;
use ic_web3_rs::signing::keccak256;
use serde::Deserialize;
use std::{borrow::Cow, collections::HashMap};

#[derive(CandidType, Deserialize, Default, Clone)]
pub struct RelayConfig {
    pub destination: Destination,
    pub items: Vec<RelayItem>,
}

#[derive(CandidType, Deserialize, Default, Clone)]
pub struct Destination {
    pub address: String,
    pub rpc_url: String,
    pub chain_id: u32,
}

#[derive(CandidType, Deserialize, Default, Clone)]
pub struct RelayItem {
    pub requester_id: String,
    pub detination_type: String,
    pub source: DataSource,
    pub plugins: Vec<(Principal, HashMap<String, String>)>,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct DataSource {
    pub canister_id: Principal,
    pub method_name: String,
    pub args: HashMap<String, String>,
}

#[derive(serde::Serialize)]
struct ItemKey {
    issuer: Principal,
    requester_id: String,
    source_canister_id: Principal,
    method_name: String,
    args: HashMap<String, String>,
}

impl Default for DataSource {
    fn default() -> Self {
        Self {
            canister_id: Principal::anonymous(),
            method_name: "".to_string(),
            args: HashMap::new(),
        }
    }
}
impl Storable for RelayConfig {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl Storable for Destination {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl Storable for RelayItem {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl Storable for DataSource {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl RelayItem {
    pub fn key(&self, issuer: Principal) -> [u8; 32usize] {
        let key = ItemKey {
            issuer,
            requester_id: self.requester_id.clone(),
            source_canister_id: self.source.canister_id.clone(),
            method_name: self.source.method_name.clone(),
            args: self.source.args.clone(),
        };
        keccak256(
            serde_json::to_vec(&key)
                .expect("failed to serialize")
                .as_slice(),
        )
    }
}
