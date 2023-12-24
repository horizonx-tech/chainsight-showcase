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
    pub block: u32,
    pub log_index: u32,
    pub txn_index: u32,
    pub timestamp: u32,
    pub event_count: u32,
    pub address: String,
    pub token0: String,
    pub token1: String,
    pub fee: u32,
    pub protocol_token0_fees: u32,
    pub protocol_token1_fees: u32,
    pub sqrt_ratio_x96: u32,
    pub liquidity: u32,
    pub tick_current: i32,
    pub tick_spacing: i32,
    pub ticks: String,
    pub fee_growth_global0_x128: u32,
    pub fee_growth_global1_x128: u32,
    pub max_liquidity_per_tick: u32,
    pub token0_price: u32,
    pub token1_price: u32,
    pub token0_amount: u32,
    pub token1_amount: u32,
    pub create_event: String,
    pub positions: String,
    pub uncollected_fees_token0: u32,
    pub uncollected_fees_token1: u32,
}

// #[derive(Debug, Clone, candid::CandidType, candid::Deserialize, serde::Serialize, chainsight_cdk_macros::StableMemoryStorable)]
// pub struct Ticks {}
