// Auto-generated code from manifest.
// You update the structure as needed.
// The existence of the SnapshotValue structure must be maintained.
use candid::{Decode, Encode};
#[derive(Debug, Clone, candid::CandidType, candid::Deserialize, serde::Serialize, chainsight_cdk_macros::StableMemoryStorable)]
pub struct SnapshotValue {
    #[serde(rename = "internet-computer")]
    pub internet_computer: InternetComputer,
}

#[derive(Debug, Clone, candid::CandidType, candid::Deserialize, serde::Serialize, chainsight_cdk_macros::StableMemoryStorable)]
pub struct InternetComputer {
    pub usd: f64,
    pub usd_market_cap: f64,
    pub usd_24h_vol: f64,
    pub usd_24h_change: f64,
    pub eur: f64,
    pub eur_market_cap: f64,
    pub eur_24h_vol: f64,
    pub eur_24h_change: f64,
    pub btc: f64,
    pub btc_market_cap: f64,
    pub btc_24h_vol: f64,
    pub btc_24h_change: f64,
    pub eth: f64,
    pub eth_market_cap: f64,
    pub eth_24h_vol: f64,
    pub eth_24h_change: f64,
    pub last_updated_at: i64,
}
