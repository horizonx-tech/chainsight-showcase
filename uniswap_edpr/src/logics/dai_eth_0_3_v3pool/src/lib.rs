// Auto-generated code from manifest.
// You update the structure as needed.
// The existence of the SnapshotValue structure must be maintained.
use candid::{Decode, Encode};
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

#[derive(Debug, Clone, candid::CandidType, candid::Deserialize, serde::Serialize, chainsight_cdk_macros::StableMemoryStorable)]
pub struct SnapshotValue {
  pub jsonrpc: String,
  pub id: String,
  pub result: _ResultX,
}

#[derive(Debug, Clone, candid::CandidType, candid::Deserialize, serde::Serialize, chainsight_cdk_macros::StableMemoryStorable)]
pub struct _ResultX {
  // pub block: u32,
  // pub log_index: u32,
  // pub txn_index: u32,
  // pub timestamp: u32,
  // pub event_count: u32,
  pub address: String,
  pub token0: String,
  // pub token1: String,
  // pub fee: u32,
  // pub protocol_token0_fees: String,
  // pub protocol_token1_fees: String,
  pub sqrt_ratio_x96: String,
  pub liquidity: String,
  pub tick_current: i32,
  pub tick_spacing: i32,
  #[serde(default)]
  #[serde(deserialize_with = "deserialize_ticks")]
  pub ticks: HashMap<String, Tick>,
  // pub fee_growth_global0_x128: String,
  // pub fee_growth_global1_x128: String,
  // pub max_liquidity_per_tick: String,
  // pub token0_price: String,
  // pub token1_price: String,
  // pub token0_amount: String,
  // pub token1_amount: String,
  // pub create_event: Option<HashMap<String, String>>,
  // pub positions: HashMap<String, String>,
  // pub uncollected_fees_token0: String,
  // pub uncollected_fees_token1: String,
}

#[derive(Debug, Clone, candid::CandidType, candid::Deserialize, serde::Serialize, chainsight_cdk_macros::StableMemoryStorable)]
pub struct Tick {
  #[serde(deserialize_with = "deserialize_index")]
  index: String,
  // liquidity_gross: String,
  liquidity_net: String,
  // fee_growth_outside_0x128: String,
  // fee_growth_outside_1x128: String,
  // initialized: bool,
}

fn deserialize_ticks<'de, D>(deserializer: D) -> Result<HashMap<String, Tick>, D::Error>
where
  D: serde::Deserializer<'de>,
{
  let ticks_map: HashMap<String, Tick> = Deserialize::deserialize(deserializer)?;
  Ok(ticks_map)
}

fn deserialize_index<'de, D>(deserializer: D) -> Result<String, D::Error>
where
  D: serde::Deserializer<'de>,
{
  let index: Result<i32, _> = Deserialize::deserialize(deserializer);
  Ok(index.map(|i| i.to_string()).unwrap_or_default())
}