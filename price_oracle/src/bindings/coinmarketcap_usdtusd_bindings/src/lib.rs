// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Deserialize, Principal, Encode, Decode};


#[derive(CandidType, Deserialize)]
pub struct CanisterMetricsSnapshot {
  pub pub cycles: candid::Nat,
  pub pub timestamp: u64,
}

#[derive(CandidType, Deserialize)]
pub enum Env { Production, Test, LocalDevelopment }

#[derive(CandidType, Deserialize)]
pub enum InitError {
  InvalidDestination(String),
  InvalidPrincipal(Principal),
  InvalidContent(String),
  InvalidRequest(String),
}

#[derive(CandidType, Deserialize)]
enum Result_ { Ok, Err(InitError) }

#[derive(CandidType, Deserialize)]
pub struct Snapshot { pub pub value: Box<SnapshotValue>, pub pub timestamp: u64 }

#[derive(CandidType, Deserialize)]
pub struct SnapshotValue { pub pub dummy: u64 }



use chainsight_cdk :: lens :: LensFinder ; use chainsight_cdk_macros :: algorithm_lens_finder ; async fn _get_target_proxy (target : candid :: Principal) -> candid :: Principal { let out : ic_cdk :: api :: call :: CallResult < (candid :: Principal ,) > = ic_cdk :: api :: call :: call (target , "get_proxy" , ()) . await ; out . unwrap () . 0 } algorithm_lens_finder ! ("coinmarketcap_usdtusd" , "proxy_get_last_snapshot_value" , SnapshotValue) ;