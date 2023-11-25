// Auto-generated code from manifest.
// You update the structure as needed.
// The existence of the SnapshotValue structure must be maintained.
use candid::{Decode, Encode};
#[derive(
    Debug,
    Clone,
    candid::CandidType,
    candid::Deserialize,
    serde::Serialize,
    chainsight_cdk_macros::StableMemoryStorable,
)]
pub struct SnapshotValue {
    pub chart: Chart,
}

#[derive(
    Debug,
    Clone,
    candid::CandidType,
    candid::Deserialize,
    serde::Serialize,
    chainsight_cdk_macros::StableMemoryStorable,
)]
pub struct Chart {
    pub result: Vec<Result_>,
}

#[derive(
    Debug,
    Clone,
    candid::CandidType,
    candid::Deserialize,
    serde::Serialize,
    chainsight_cdk_macros::StableMemoryStorable,
)]
pub struct Result_ {
    pub meta: Meta,
}

#[derive(
    Debug,
    Clone,
    candid::CandidType,
    candid::Deserialize,
    serde::Serialize,
    chainsight_cdk_macros::StableMemoryStorable,
)]
pub struct Meta {
    pub currency: String,
    #[serde(rename = "regularMarketPrice")]
    pub regular_market_price: f64,
}
