use candid::{Decode, Encode};
use chainsight_cdk_macros::StableMemoryStorable;
#[derive(
    Debug,
    Clone,
    candid :: CandidType,
    candid :: Deserialize,
    serde :: Serialize,
    StableMemoryStorable,
)]
pub struct SnapshotValue {
    pub oasys: Inner,
}
#[derive(
    Debug,
    Clone,
    candid :: CandidType,
    candid :: Deserialize,
    serde :: Serialize,
    StableMemoryStorable,
)]
pub struct Inner {
    pub usd: f64,
    usd_24h_vol: f64,
}
