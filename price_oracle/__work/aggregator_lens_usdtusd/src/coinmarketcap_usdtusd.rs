// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
use candid::{self, CandidType, Deserialize, Principal};

#[derive(CandidType, Deserialize)]
pub struct CanisterMetricsSnapshot {
    pub cycles: candid::Nat,
    pub timestamp: u64,
}

#[derive(CandidType, Deserialize)]
pub struct CurrencyData {
    pub quote: Box<Quote>,
}

#[derive(CandidType, Deserialize)]
pub enum Env {
    Production,
    Test,
    LocalDevelopment,
}

#[derive(CandidType, Deserialize)]
pub enum InitError {
    InvalidDestination(String),
    InvalidPrincipal(Principal),
    InvalidContent(String),
    InvalidRequest(String),
}

#[derive(CandidType, Deserialize)]
pub struct Inner {
    pub data: CurrencyData,
}

#[derive(CandidType, Deserialize)]
pub struct Quote {
    pub usd: Box<QuoteData>,
}

#[derive(CandidType, Deserialize)]
pub struct QuoteData {
    pub volume_24h: f64,
    pub price: f64,
}

#[derive(CandidType, Deserialize)]
enum Result {
    Ok,
    Err(InitError),
}

#[derive(CandidType, Deserialize)]
pub struct Snapshot {
    pub value: Box<SnapshotValue>,
    pub timestamp: u64,
}

#[derive(CandidType, Deserialize)]
pub struct SnapshotValue {
    pub data: Inner,
}
