// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
use candid::{self, CandidType, Deserialize, Principal};

#[derive(CandidType, Deserialize)]
pub struct CanisterMetricsSnapshot {
    cycles: candid::Nat,
    timestamp: u64,
}

#[derive(CandidType, Deserialize)]
pub enum Env {
    Production,
    Test,
    LocalDevelopment,
}

#[derive(CandidType, Deserialize)]
pub struct HttpHeader {
    value: String,
    name: String,
}

#[derive(CandidType, Deserialize)]
pub struct HttpResponse {
    status: candid::Nat,
    body: serde_bytes::ByteBuf,
    headers: Vec<HttpHeader>,
}

#[derive(CandidType, Deserialize)]
pub enum InitError {
    InvalidDestination(String),
    InvalidPrincipal(Principal),
    InvalidContent(String),
    InvalidRequest(String),
}

#[derive(CandidType, Deserialize)]
enum Result {
    Ok,
    Err(InitError),
}

#[derive(CandidType, Deserialize)]
enum Result_1 {
    Ok,
    Err(String),
}

#[derive(CandidType, Deserialize)]
pub struct Snapshot {
    value: String,
    timestamp: u64,
}

#[derive(CandidType, Deserialize)]
pub enum SourceType {
    evm,
    chainsight,
}

#[derive(CandidType, Deserialize)]
pub struct Sources {
    source: String,
    interval_sec: Option<u32>,
    attributes: Box<Web3AlgorithmIndexerSourceAttrs>,
    source_type: SourceType,
}

#[derive(CandidType, Deserialize)]
pub struct TransformArgs {
    context: serde_bytes::ByteBuf,
    response: HttpResponse,
}

#[derive(CandidType, Deserialize)]
pub struct Web3AlgorithmIndexerSourceAttrs {
    chain_id: u64,
    function_name: String,
}

#[derive(CandidType, Deserialize)]
pub struct Web3CtxParam {
    env: Env,
    url: String,
    from: Option<String>,
    chain_id: u64,
}
