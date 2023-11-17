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
    pub market_data: MarketData,
}

#[derive(
    Debug,
    Clone,
    candid::CandidType,
    candid::Deserialize,
    serde::Serialize,
    chainsight_cdk_macros::StableMemoryStorable,
)]
pub struct MarketData {
    pub market_cap: MarketCap,
    pub total_supply: f64,
    pub circulating_supply: f64,
}

#[derive(
    Debug,
    Clone,
    candid::CandidType,
    candid::Deserialize,
    serde::Serialize,
    chainsight_cdk_macros::StableMemoryStorable,
)]
pub struct CurrentPrice {
    pub usd: i64,
}

#[derive(
    Debug,
    Clone,
    candid::CandidType,
    candid::Deserialize,
    serde::Serialize,
    chainsight_cdk_macros::StableMemoryStorable,
)]
pub struct MarketCap {
    pub usd: i64,
}

#[derive(
    Debug,
    Clone,
    candid::CandidType,
    candid::Deserialize,
    serde::Serialize,
    chainsight_cdk_macros::StableMemoryStorable,
)]
pub struct High24h {
    pub usd: i64,
}

#[derive(
    Debug,
    Clone,
    candid::CandidType,
    candid::Deserialize,
    serde::Serialize,
    chainsight_cdk_macros::StableMemoryStorable,
)]

pub struct Low24h {
    pub usd: i64,
}
