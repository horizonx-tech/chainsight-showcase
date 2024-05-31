#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Decode, Deserialize, Encode, Principal};
#[derive(
    Clone,
    Debug,
    candid :: CandidType,
    candid :: Deserialize,
    serde :: Serialize,
    chainsight_cdk_macros :: StableMemoryStorable,
)]
pub struct CycleManagement {
    pub refueling_amount: u128,
    pub initial_supply: u128,
    pub refueling_threshold: u128,
}
#[derive(
    Clone,
    Debug,
    candid :: CandidType,
    candid :: Deserialize,
    serde :: Serialize,
    chainsight_cdk_macros :: StableMemoryStorable,
)]
pub struct CycleManagements {
    pub db: CycleManagement,
    pub vault_intial_supply: u128,
    pub refueling_interval: u64,
    pub proxy: CycleManagement,
    pub indexer: CycleManagement,
}
#[derive(
    Clone,
    Debug,
    candid :: CandidType,
    candid :: Deserialize,
    serde :: Serialize,
    chainsight_cdk_macros :: StableMemoryStorable,
)]
pub enum Env {
    Production,
    Test,
    LocalDevelopment,
}
#[derive(
    Clone,
    Debug,
    candid :: CandidType,
    candid :: Deserialize,
    serde :: Serialize,
    chainsight_cdk_macros :: StableMemoryStorable,
)]
pub enum InitError {
    InvalidDestination(String),
    InvalidPrincipal(Principal),
    InvalidContent(String),
    InvalidRequest(String),
}
#[derive(
    Clone,
    Debug,
    candid :: CandidType,
    candid :: Deserialize,
    serde :: Serialize,
    chainsight_cdk_macros :: StableMemoryStorable,
)]
pub struct LensArgs {
    pub args: Vec<String>,
    pub targets: Vec<String>,
}
pub type RequestArgsType = LensArgs;
#[derive(
    Clone,
    Debug,
    candid :: CandidType,
    candid :: Deserialize,
    serde :: Serialize,
    chainsight_cdk_macros :: StableMemoryStorable,
)]
pub enum Result_ {
    Ok,
    Err(InitError),
}
#[derive(
    Clone,
    Debug,
    candid :: CandidType,
    candid :: Deserialize,
    serde :: Serialize,
    chainsight_cdk_macros :: StableMemoryStorable,
)]
pub enum SourceType {
    #[serde(rename = "evm")]
    Evm,
    #[serde(rename = "https")]
    Https,
    #[serde(rename = "chainsight")]
    Chainsight,
}
#[derive(
    Clone,
    Debug,
    candid :: CandidType,
    candid :: Deserialize,
    serde :: Serialize,
    chainsight_cdk_macros :: StableMemoryStorable,
)]
pub struct Sources {
    pub source: String,
    pub interval_sec: Option<u32>,
    pub attributes: Vec<(String, String)>,
    pub source_type: SourceType,
}
