#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Deserialize, Principal, Encode, Decode};
#[derive(Clone, Debug, candid :: CandidType, candid :: Deserialize, serde :: Serialize, chainsight_cdk_macros :: StableMemoryStorable)]
pub struct BatchQueryParams { pub params: Vec<(String,Box<QueryOptions>,)> }
#[derive(Clone, Debug, candid :: CandidType, candid :: Deserialize, serde :: Serialize, chainsight_cdk_macros :: StableMemoryStorable)]
pub struct CycleManagement {
  pub refueling_amount: u128,
  pub initial_supply: u128,
  pub refueling_threshold: u128,
}
#[derive(Clone, Debug, candid :: CandidType, candid :: Deserialize, serde :: Serialize, chainsight_cdk_macros :: StableMemoryStorable)]
pub struct CycleManagements {
  pub db: CycleManagement,
  pub vault_intial_supply: u128,
  pub refueling_interval: u64,
  pub proxy: CycleManagement,
  pub indexer: CycleManagement,
}
#[derive(Clone, Debug, candid :: CandidType, candid :: Deserialize, serde :: Serialize, chainsight_cdk_macros :: StableMemoryStorable)]
pub enum Env { Production, Test, LocalDevelopment }
#[derive(Clone, Debug, candid :: CandidType, candid :: Deserialize, serde :: Serialize, chainsight_cdk_macros :: StableMemoryStorable)]
pub struct HttpHeader { pub value: String, pub name: String }
#[derive(Clone, Debug, candid :: CandidType, candid :: Deserialize, serde :: Serialize, chainsight_cdk_macros :: StableMemoryStorable)]
pub struct HttpResponse {
  pub status: u128,
  pub body: serde_bytes::ByteBuf,
  pub headers: Vec<HttpHeader>,
}
#[derive(Clone, Debug, candid :: CandidType, candid :: Deserialize, serde :: Serialize, chainsight_cdk_macros :: StableMemoryStorable)]
pub struct HttpsSnapshotIndexerSourceAttrs {
  pub queries: Vec<(String,String,)>,
}
#[derive(Clone, Debug, candid :: CandidType, candid :: Deserialize, serde :: Serialize, chainsight_cdk_macros :: StableMemoryStorable)]
pub enum InitError {
  InvalidDestination(String),
  InvalidPrincipal(Principal),
  InvalidContent(String),
  InvalidRequest(String),
}
#[derive(Clone, Debug, candid :: CandidType, candid :: Deserialize, serde :: Serialize, chainsight_cdk_macros :: StableMemoryStorable)]
pub struct QueryOptions {
  pub from_timestamp: Option<i64>,
  pub to_timestamp: Option<i64>,
}
pub type RequestArgsType = Vec<String>;
pub type ResponseType = Vec<(String,Option<Box<Snapshot>>,)>;
#[derive(Clone, Debug, candid :: CandidType, candid :: Deserialize, serde :: Serialize, chainsight_cdk_macros :: StableMemoryStorable)]
pub enum Result_ { Ok, Err(InitError) }
#[derive(Clone, Debug, candid :: CandidType, candid :: Deserialize, serde :: Serialize, chainsight_cdk_macros :: StableMemoryStorable)]
pub enum Result1 { Ok, Err(String) }
#[derive(Clone, Debug, candid :: CandidType, candid :: Deserialize, serde :: Serialize, chainsight_cdk_macros :: StableMemoryStorable)]
pub struct Snapshot {
  pub id: Box<SnapshotId>,
  pub value: Box<SnapshotValue>,
  pub timestamp: u64,
}
#[derive(Clone, Debug, candid :: CandidType, candid :: Deserialize, serde :: Serialize, chainsight_cdk_macros :: StableMemoryStorable)]
pub struct SnapshotId { pub id: String }
#[derive(Clone, Debug, candid :: CandidType, candid :: Deserialize, serde :: Serialize, chainsight_cdk_macros :: StableMemoryStorable)]
pub struct SnapshotValue { pub raw: serde_bytes::ByteBuf }
#[derive(Clone, Debug, candid :: CandidType, candid :: Deserialize, serde :: Serialize, chainsight_cdk_macros :: StableMemoryStorable)]
pub enum SourceType {
  #[serde(rename="evm")]
  Evm,
  #[serde(rename="https")]
  Https,
  #[serde(rename="chainsight")]
  Chainsight,
}
#[derive(Clone, Debug, candid :: CandidType, candid :: Deserialize, serde :: Serialize, chainsight_cdk_macros :: StableMemoryStorable)]
pub struct Sources {
  pub source: String,
  pub interval_sec: Option<u32>,
  pub attributes: HttpsSnapshotIndexerSourceAttrs,
  pub source_type: SourceType,
}
#[derive(Clone, Debug, candid :: CandidType, candid :: Deserialize, serde :: Serialize, chainsight_cdk_macros :: StableMemoryStorable)]
pub struct TransformArgs {
  pub context: serde_bytes::ByteBuf,
  pub response: HttpResponse,
}