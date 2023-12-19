// Auto-generated code from manifest.
// You update the structure as needed.
// The existence of the SnapshotValue structure must be maintained.
use candid::{Decode, Encode};
#[derive(Debug, Clone, candid::CandidType, candid::Deserialize, serde::Serialize, chainsight_cdk_macros::StableMemoryStorable)]
pub struct SnapshotValue {
    pub last_updated: String,
    pub current: Vec<(String, f64)>,
    pub attr: Vec<Vec<String>>,
    pub month_ago: Vec<(String, f64)>,
    pub year_ago: Vec<(String, f64)>,
}
