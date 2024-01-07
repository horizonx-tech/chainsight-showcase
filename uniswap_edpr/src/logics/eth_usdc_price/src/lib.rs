// Auto-generated code from manifest.
// You update the structure as needed.
// The existence of the SnapshotValue structure must be maintained.
use candid::{Decode, Encode};
#[derive(Debug, Clone, candid::CandidType, candid::Deserialize, serde::Serialize, chainsight_cdk_macros::StableMemoryStorable)]
pub struct SnapshotValue {
    pub jsonrpc: String,
    pub id: String,
    pub result: _ResultX,
}

#[derive(Debug, Clone, candid::CandidType, candid::Deserialize, serde::Serialize, chainsight_cdk_macros::StableMemoryStorable)]
pub struct _ResultX {
    pub pool: String,
    #[serde(rename = "Timestamp")]
    pub timestamp: String,
    pub token_info: TokenInfo,
    pub t0_price_usd: f32,
    pub t1_price_usd: f32,
}

#[derive(Debug, Clone, candid::CandidType, candid::Deserialize, serde::Serialize, chainsight_cdk_macros::StableMemoryStorable)]
pub struct TokenInfo {
    pub pool: String,
    pub t0_address: String,
    pub t0_symbol: String,
    pub t0_name: String,
    pub t0_decimals: i64,
    pub t0_totalsupply: String,
    pub t1_address: String,
    pub t1_symbol: String,
    pub t1_name: String,
    pub t1_decimals: i64,
    pub t1_totalsupply: String,
}
