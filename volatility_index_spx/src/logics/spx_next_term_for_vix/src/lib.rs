// Auto-generated code from manifest.
// You can implement the function to get the query parameters.
use std::collections::BTreeMap;

const SECS_FOR_DAY: u64 = 24 * 60 * 60;
const SECS_FOR_WEEK: u64 = SECS_FOR_DAY * 7;

pub fn get_query_parameters() -> BTreeMap<String, String> {
    let now_secs = ic_cdk::api::time() / (1000 * 1000000);
    let current_day = now_secs / SECS_FOR_DAY * SECS_FOR_DAY;
    let rounded_epoch = current_day / SECS_FOR_WEEK * SECS_FOR_WEEK;
    let fri = rounded_epoch + SECS_FOR_DAY;
    let next_term = if current_day - rounded_epoch < 6 {
        fri + SECS_FOR_WEEK * 5
    } else {
        fri + SECS_FOR_WEEK * 6
    };

    ic_cdk::println!("next_term: {}", next_term);

    BTreeMap::from([
        ("straddle".to_string(), "false".to_string()),
        ("date".to_string(), next_term.to_string()),
    ])
}

// You update the structure as needed.
// The existence of the SnapshotValue structure must be maintained.
use candid::{Decode, Encode};
#[derive(Debug, Clone, candid::CandidType, candid::Deserialize, serde::Serialize, chainsight_cdk_macros::StableMemoryStorable)]
pub struct SnapshotValue {
    #[serde(rename = "optionChain")]
    pub option_chain: OptionChain,
}

#[derive(Debug, Clone, candid::CandidType, candid::Deserialize, serde::Serialize, chainsight_cdk_macros::StableMemoryStorable)]
pub struct OptionChain {
    pub result: Vec<ResultSpx>,
}

#[derive(Debug, Clone, candid::CandidType, candid::Deserialize, serde::Serialize, chainsight_cdk_macros::StableMemoryStorable)]
pub struct ResultSpx {
    #[serde(rename = "underlyingSymbol")]
    pub underlying_symbol: String,
    #[serde(rename = "expirationDates")]
    pub expiration_dates: Vec<i64>,
    pub strikes: Vec<f64>,
    pub options: Vec<OptionSpx>,
}

#[derive(Debug, Clone, candid::CandidType, candid::Deserialize, serde::Serialize, chainsight_cdk_macros::StableMemoryStorable)]
pub struct OptionSpx {
    #[serde(rename = "expirationDate")]
    pub expiration_date: i64,
    pub calls: Vec<OptionSpxInner>,
    pub puts: Vec<OptionSpxInner>,
}

#[derive(Debug, Clone, candid::CandidType, candid::Deserialize, serde::Serialize, chainsight_cdk_macros::StableMemoryStorable)]
pub struct OptionSpxInner {
    #[serde(rename = "contractSymbol")]
    pub contract_symbol: String,
    pub strike: f64,
    pub currency: String,
    #[serde(rename = "lastPrice")]
    pub last_price: f64,
    pub bid: f64,
    pub ask: f64,
    pub expiration: i64,
}
