use candid::{Decode, Encode};
use chainsight_cdk_macros::StableMemoryStorable;
#[derive(
    Debug,
    Clone,
    candid::CandidType,
    candid::Deserialize,
    serde::Serialize,
    StableMemoryStorable,
    PartialEq,
)]
pub struct SnapshotValue {
    pub data: Inner,
}
#[derive(
    Debug,
    Clone,
    candid::CandidType,
    candid::Deserialize,
    serde::Serialize,
    StableMemoryStorable,
    PartialEq,
)]
pub struct Inner {
    #[serde(alias = "22265", alias = "data")]
    pub data: CurrencyData,
}
#[derive(
    Debug,
    Clone,
    candid::CandidType,
    candid::Deserialize,
    serde::Serialize,
    StableMemoryStorable,
    PartialEq,
)]
pub struct CurrencyData {
    pub quote: Quote,
}
#[derive(
    Debug,
    Clone,
    candid::CandidType,
    candid::Deserialize,
    serde::Serialize,
    StableMemoryStorable,
    PartialEq,
)]
pub struct Quote {
    #[serde(alias = "USD", alias = "usd")]
    pub usd: QuoteData,
}
#[derive(
    Debug,
    Clone,
    candid::CandidType,
    candid::Deserialize,
    serde::Serialize,
    StableMemoryStorable,
    PartialEq,
)]
pub struct QuoteData {
    price: f64,
    volume_24h: f64,
}