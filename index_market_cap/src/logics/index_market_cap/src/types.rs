use std::collections::HashMap;

use candid::{ser::IDLBuilder, CandidType, Decode, Principal};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct BulkSnapshotIndexerHttps {
    pub principal: Principal,
}
impl BulkSnapshotIndexerHttps {
    pub fn new(principal: Principal) -> Self {
        Self { principal }
    }
    pub async fn batch_get(
        &self,
        ids: Vec<String>,
    ) -> Result<HashMap<String, Option<Snapshot>>, String> {
        let args: Vec<u8> = IDLBuilder::new()
            .arg(&ids)
            .map_err(|e| e.to_string())?
            .serialize_to_vec()
            .map_err(|e| e.to_string())?;
        raw_call_target(self.principal, "batch_get_value", args).await
    }
}
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Snapshot {
    pub id: SnapshotId,
    pub value: SnapshotValue,
    pub timestamp: u64,
}

impl Default for Snapshot {
    fn default() -> Self {
        let value = 0.0_f64;
        Snapshot {
            id: SnapshotId { id: "".to_string() },
            value: SnapshotValue {
                raw: bincode::serialize(&value).unwrap(),
            },
            timestamp: 0,
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct SnapshotValue {
    pub raw: Vec<u8>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
struct QueryOptions {
    from_timestamp: Option<i64>,
    to_timestamp: Option<i64>,
}

impl Snapshot {
    pub fn value(&self) -> Option<f64> {
        let value_f64: Result<Value, bincode::Error> =
            bincode::deserialize(&self.value.raw.as_slice());
        if value_f64.is_err() {
            ic_cdk::println!("Failed to deserialize value: {:?}", value_f64.err());
            return None;
        }
        Some(value_f64.unwrap().v)
    }
    pub fn value_from_string(&self) -> Option<f64> {
        let value_string: Result<DexValue, bincode::Error> =
            bincode::deserialize(&self.value.raw.as_slice());
        if value_string.is_err() {
            ic_cdk::println!("Failed to deserialize value: {:?}", value_string.err());
            return None;
        }
        Some(value_string.unwrap().v.parse().unwrap_or_default())
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct SnapshotId {
    pub id: String,
}

#[derive(Deserialize, Serialize)]
struct Value {
    v: f64,
}
#[derive(Deserialize, Serialize)]
struct DexValue {
    v: String,
}
async fn raw_call_target<T: CandidType + DeserializeOwned>(
    target: Principal,
    method_name: &str,
    args: Vec<u8>,
) -> Result<T, String> {
    let result: Result<Vec<u8>, (ic_cdk::api::call::RejectionCode, String)> =
        ic_cdk::api::call::call_raw(target, method_name, args, 0).await;
    match result {
        Ok(bytes) => {
            Decode!(bytes.as_slice(), T).map_err(|e| format!("Error decoding response: {:?}", e))
        }
        Err((code, msg)) => Err(format!("Error: {:?}, {:?}", code, msg)),
    }
}
