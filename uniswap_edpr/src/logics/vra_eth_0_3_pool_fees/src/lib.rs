// Auto-generated code from manifest.
// You update the structure as needed.
// The existence of the SnapshotValue structure must be maintained.
use candid::{Decode, Encode};
#[derive(Debug, Clone, candid::CandidType, candid::Deserialize, serde::Serialize, chainsight_cdk_macros::StableMemoryStorable)]
pub struct SnapshotValue {
    pub jsonrpc: String,
    pub id: String,
    pub result: Vec<_ResultX>,
}

#[derive(Debug, Clone, candid::CandidType, candid::Deserialize, serde::Serialize, chainsight_cdk_macros::StableMemoryStorable)]
pub struct _ResultX {
    pub fees_24h_usd: f32,
    pub volume_24h_usd: f32,
    pub pool_summary_level_1: PoolSummaryLevel1,
}

#[derive(Debug, Clone, candid::CandidType, candid::Deserialize, serde::Serialize, chainsight_cdk_macros::StableMemoryStorable)]
pub struct PoolSummaryLevel1 {
    pub block: i64,
    pub address: String,
    pub fee: i64,
    pub t0: String,
    pub t0_name: String,
    pub t0_symbol: String,
    pub t0_decimals: i64,
    pub t1: String,
    pub t1_name: String,
    pub t1_symbol: String,
    pub t1_decimals: i64,
    pub is_preferred_token_order: bool,
    pub default_granularities: Vec<i64>,
    pub default_flipped_granularities: Vec<i64>,
    pub last_price: f64,
    pub last_amount: f64,
    pub last_side: String,
    pub t0_tvl: f64,
    pub t0_price: f64,
    pub t0_volume: f64,
    pub t0_change: f64,
    pub t0_tvl_usd: f64,
    pub t0_low: f64,
    pub t0_high: f64,
    pub t0_price_usd: i64,
    pub t0_volume_usd: f64,
    pub t0_change_usd: i64,
    pub t0_low_usd: f64,
    pub t0_high_usd: f64,
    pub t0_fees_usd: f64,
    pub t1_tvl: f64,
    pub t1_price: f64,
    pub t1_volume: f64,
    pub t1_change: f64,
    pub t1_low: f64,
    pub t1_high: f64,
    pub t1_tvl_usd: f64,
    pub t1_price_usd: f64,
    pub t1_volume_usd: f64,
    pub t1_change_usd: f64,
    pub t1_low_usd: f64,
    pub t1_high_usd: f64,
    pub t1_fees_usd: f64,
    pub tx_count: i64,
    pub tvl_usd: f64,
    pub total_fees_usd: f64,
    pub tvl_usd_change: f64,
    pub t0_volume_change: f64,
    pub t1_volume_change: f64,
    pub t0_volume_7d: f64,
    pub t1_volume_7d: f64,
    pub t0_volume_change_7d: f64,
    pub t1_volume_change_7d: f64,
    pub total_volume_7d_usd: f64,
    pub last_updated: i64,
    pub cleaned: bool,
}
