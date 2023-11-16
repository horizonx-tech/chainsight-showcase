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
    pub id: String,
    pub symbol: String,
    pub market_data: MarketData,
    pub last_updated: String,
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
    pub current_price: CurrentPrice,
    pub market_cap: MarketCap,
    pub fully_diluted_valuation: FullyDilutedValuation,
    pub total_volume: TotalVolume,
    pub total_supply: f64,
    pub max_supply: f64,
    pub circulating_supply: f64,
    pub last_updated: String,
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
pub struct FullyDilutedValuation {
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
pub struct TotalVolume {
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
