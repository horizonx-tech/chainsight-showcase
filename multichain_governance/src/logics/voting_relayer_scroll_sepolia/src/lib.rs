mod types;
pub type CallCanisterResponse = types::ResponseType;
pub type LensArgs = voting_relayer_scroll_sepolia_bindings::LensArgs;
pub fn call_args() -> (u64, u64) {
    voting_relayer_sepolia::call_args()
}

pub type ContractCallArgs = voting_relayer_sepolia::ContractCallArgs;
pub fn convert(res: &CallCanisterResponse) -> ContractCallArgs {
    voting_relayer_sepolia::convert(&voting_relayer_sepolia::CallCanisterResponse {
        0: res.0.clone(),
        1: res.1.clone(),
        2: res.2.clone(),
        3: res.3.clone(),
        4: res.4.clone(),
        5: res.5.clone(),
    })
}
pub fn filter(res: &CallCanisterResponse) -> bool {
    voting_relayer_sepolia::filter(&voting_relayer_sepolia::CallCanisterResponse {
        0: res.0.clone(),
        1: res.1.clone(),
        2: res.2.clone(),
        3: res.3.clone(),
        4: res.4.clone(),
        5: res.5.clone(),
    })
}
