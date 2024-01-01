// Auto-generated code from manifest.
// You update the structure as needed.
// The existence of the SnapshotValue structure must be maintained.
use candid::{Decode, Encode};
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use serde_json :: Value;

fn deserialize_nested_object<'de, D>(deserializer: D) -> Result<HashMap<String, String>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(HashMap::deserialize(deserializer)?)
}

#[derive(Debug, Clone, candid::CandidType, candid::Deserialize, serde::Serialize, chainsight_cdk_macros::StableMemoryStorable)]
pub struct SnapshotValue {
  pub jsonrpc: String,
  pub id: String,
  pub result: _ResultX,
}

#[derive(Debug, Clone, candid::CandidType, candid::Deserialize, serde::Serialize, serde_json::Value, chainsight_cdk_macros::StableMemoryStorable)]
pub struct _ResultX {
  pub block: u32,
  pub log_index: u32,
  pub txn_index: u32,
  pub timestamp: u32,
  pub event_count: u32,
  pub address: String,
  pub token0: String,
  pub token1: String,
  pub fee: u32,
  pub protocol_token0_fees: String,
  pub protocol_token1_fees: String,
  pub sqrt_ratio_x96: String,
  pub liquidity: String,
  pub tick_current: i32,
  pub tick_spacing: i32,
  #[serde(deserialize_with = "deserialize_nested_object")]
  pub ticks: HashMap<String, Value>,
  pub fee_growth_global0_x128: String,
  pub fee_growth_global1_x128: String,
  pub max_liquidity_per_tick: String,
  pub token0_price: String,
  pub token1_price: String,
  pub token0_amount: String,
  pub token1_amount: String,
  // pub create_event: Option<HashMap<String, Value>>,
  // pub positions: HashMap<String, Value>,
  pub uncollected_fees_token0: String,
  pub uncollected_fees_token1: String,
}
