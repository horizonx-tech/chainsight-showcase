// Auto-generated code from manifest.
// You update the structure as needed.
// The existence of the SnapshotValue structure must be maintained.
use candid::{Decode, Encode};
use serde::{Deserialize, Deserializer};
use serde_json::Value;
#[derive(
    Debug,
    Clone,
    candid::CandidType,
    candid::Deserialize,
    serde::Serialize,
    chainsight_cdk_macros::StableMemoryStorable,
)]
pub struct SnapshotValue {
    pub data: Inner,
}

#[derive(
    Debug,
    Clone,
    candid::CandidType,
    candid::Deserialize,
    serde::Serialize,
    chainsight_cdk_macros::StableMemoryStorable,
)]
pub struct Inner {
    #[serde(alias = "1", alias = "_1")]
    pub _1: CurrencyData,
    #[serde(alias = "2", alias = "_2")]
    pub _2: CurrencyData,
    #[serde(alias = "52", alias = "_3")]
    pub _3: CurrencyData,
    #[serde(alias = "74", alias = "_4")]
    pub _4: CurrencyData,
    #[serde(alias = "825", alias = "_5")]
    pub _5: CurrencyData,
    #[serde(alias = "1027", alias = "_6")]
    pub _6: CurrencyData,
    #[serde(alias = "1831", alias = "_7")]
    pub _7: CurrencyData,
    #[serde(alias = "1839", alias = "_8")]
    pub _8: CurrencyData,
    #[serde(alias = "1958", alias = "_9")]
    pub _9: CurrencyData,
    #[serde(alias = "1975", alias = "_10")]
    pub _10: CurrencyData,
    #[serde(alias = "2010", alias = "_11")]
    pub _11: CurrencyData,
    #[serde(alias = "3408", alias = "_12")]
    pub _12: CurrencyData,
    #[serde(alias = "3890", alias = "_13")]
    pub _13: CurrencyData,
    #[serde(alias = "5426", alias = "_14")]
    pub _14: CurrencyData,
    #[serde(alias = "5805", alias = "_15")]
    pub _15: CurrencyData,
    #[serde(alias = "5994", alias = "_16")]
    pub _16: CurrencyData,
    #[serde(alias = "6636", alias = "_17")]
    pub _17: CurrencyData,
    #[serde(alias = "7083", alias = "_18")]
    pub _18: CurrencyData,
    #[serde(alias = "11419", alias = "_19")]
    pub _19: CurrencyData,
    #[serde(alias = "4943", alias = "_20")]
    pub _20: CurrencyData,
}
#[derive(
    Debug,
    Clone,
    candid::CandidType,
    candid::Deserialize,
    serde::Serialize,
    chainsight_cdk_macros::StableMemoryStorable,
)]
pub struct CurrencyData {
    #[serde(deserialize_with = "de_f_or_u64")]
    pub circulating_supply: f64,
    #[serde(deserialize_with = "de_f_or_u64")]
    pub total_supply: f64,
    pub quote: Quote,
}
#[derive(
    Debug,
    Clone,
    candid::CandidType,
    candid::Deserialize,
    serde::Serialize,
    chainsight_cdk_macros::StableMemoryStorable,
)]
pub struct Quote {
    #[serde(alias = "USD", alias = "usd")]
    pub usd: QuoteData,
}
#[derive(
    Debug,
    Clone,
    candid::CandidType,
    candid::Deserialize,
    serde::Serialize,
    chainsight_cdk_macros::StableMemoryStorable,
)]
pub struct QuoteData {
    #[serde(deserialize_with = "de_f_or_u64")]
    market_cap: f64,
}

fn de_f_or_u64<'de, D: Deserializer<'de>>(deserializer: D) -> Result<f64, D::Error> {
    let v: Value = Deserialize::deserialize(deserializer)?;
    match v {
        Value::Number(n) => match n.as_f64() {
            Some(f) => Ok(f),
            None => Ok(n.as_u64().unwrap() as f64),
        },
        _ => panic!("Unexpected value"),
    }
}
