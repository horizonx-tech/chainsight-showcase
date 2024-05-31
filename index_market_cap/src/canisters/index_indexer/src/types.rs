use std::collections::HashMap;

use crate::Decode;
use crate::Encode;
use candid::CandidType;
use candid::Principal;
use chainsight_cdk_macros::StableMemoryStorable;
use index_indexer_bindings::LensArgs;
use index_market_cap::LensValue;
use serde::{Deserialize, Serialize};
use ulid_lib::Ulid;
const NANOS_PER_MILLIS: u64 = 1_000_000;
#[derive(CandidType, Clone, StableMemoryStorable, Serialize, Deserialize, Debug, PartialEq)]
pub struct Snapshot {
    pub id: SnapshotId,
    pub value: SnapshotValue,
}

impl Snapshot {
    pub async fn from(values: Vec<(AggregationKey, LensValue)>) -> Self {
        let id = SnapshotId::new().await;
        let value = values
            .into_iter()
            .map(|(k, v)| (k, v.value))
            .collect::<HashMap<AggregationKey, f64>>();
        Self { id, value }
    }
}

pub type SnapshotValue = HashMap<AggregationKey, f64>;
pub type TaskId = String;
pub type AggregationKey = String;
pub type IdToFetch = String;

#[derive(CandidType, Clone, StableMemoryStorable, Serialize, Deserialize)]
pub struct TaskOption {
    pub id_to_fetch: IdToFetch,
}

#[derive(CandidType, Clone, StableMemoryStorable, Serialize, Deserialize)]
pub struct Task {
    pub id: TaskId,
    pub lens: Principal,
    pub options: TaskOptions,
    pub source: Principal,
}

impl Task {
    pub fn to_lens_args(&self) -> LensArgs {
        LensArgs {
            args: self
                .options
                .options
                .iter()
                .map(|k| k.id_to_fetch.clone())
                .collect(),
            targets: vec![self.source.clone().to_string()],
        }
    }
}
#[derive(CandidType, Clone, StableMemoryStorable, Serialize, Deserialize)]
pub struct TaskOptions {
    pub options: Vec<TaskOption>,
}

#[derive(Clone, CandidType, Serialize, Deserialize)]
pub struct QueryOption {
    pub from_timestamp: Option<i64>,
    pub to_timestamp: Option<i64>,
}
#[derive(CandidType, Clone, StableMemoryStorable, Serialize, Deserialize, Debug)]
pub struct SnapshotId {
    pub id: String,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, StableMemoryStorable)]
pub struct SnapshotIds {
    ids: Vec<SnapshotId>,
}
impl SnapshotId {
    //#[cfg(not(test))]
    pub async fn new() -> Self {
        use ic_cdk::api::management_canister::main::raw_rand;

        let now_msec = ic_cdk::api::time() / NANOS_PER_MILLIS;
        let rand = raw_rand().await.unwrap();
        let rand_u128 = u128::from(rand.0.get(0).unwrap().clone());
        let id = Ulid::from_parts(now_msec, rand_u128);
        Self { id: id.to_string() }
    }
    //#[cfg(test)]
    //pub async fn new() -> Self {
    //    let time = 1234567890;
    //    Self {
    //        id: Ulid::from_parts(time, 0).to_string(),
    //    }
    //}
}

impl Ord for SnapshotId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_id = Ulid::from_string(&self.id).expect("Invalid id");
        let other_id = Ulid::from_string(&other.id).expect("Invalid id");
        self_id.cmp(&other_id)
    }
}
impl PartialOrd for SnapshotId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_id = Ulid::from_string(&self.id).expect("Invalid id");
        let other_id = Ulid::from_string(&other.id).expect("Invalid id");
        self_id.partial_cmp(&other_id)
    }
}
impl PartialEq for SnapshotId {
    fn eq(&self, other: &Self) -> bool {
        let self_id = Ulid::from_string(&self.id).expect("Invalid id");
        let other_id = Ulid::from_string(&other.id).expect("Invalid id");
        self_id.eq(&other_id)
    }
}
impl Eq for SnapshotId {
    fn assert_receiver_is_total_eq(&self) {
        let self_id = Ulid::from_string(&self.id).expect("Invalid id");
        self_id.assert_receiver_is_total_eq()
    }
}

#[derive(
    CandidType,
    Serialize,
    Deserialize,
    Clone,
    Debug,
    StableMemoryStorable,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
)]
pub struct Key {
    pub id: String,
}

impl From<String> for Key {
    fn from(id: String) -> Self {
        Key { id }
    }
}
impl Into<String> for Key {
    fn into(self) -> String {
        self.id
    }
}
#[derive(
    CandidType,
    Serialize,
    Deserialize,
    Clone,
    Debug,
    StableMemoryStorable,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
)]
#[stable_mem_storable_opts(max_size = 10000, is_fixed_size = true)]
pub struct Value {
    pub value: String,
}
#[cfg(not(test))]
pub async fn raw_rand() -> Result<Vec<u8>, String> {
    let (rand_msb,): (Vec<u8>,) =
        ic_cdk::api::call::call(Principal::management_canister(), "raw_rand", ())
            .await
            .map_err(|e| format!("{:?}", e))?;
    Ok(rand_msb)
}

#[cfg(test)]
mod tests {
    use ulid_lib::Ulid;

    #[test]
    fn test_ulid_from() {
        let now_nanosec = 1715055275000_i64 * 1_000_000;
        let rand = now_nanosec as u128;
        let id = Ulid::from_parts((now_nanosec / 1_000_000) as u64, rand);
        assert_eq!(id.timestamp_ms(), 1715055275000);
    }
}
