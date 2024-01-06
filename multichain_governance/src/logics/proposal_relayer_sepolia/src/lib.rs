mod types;
pub type CallCanisterResponse = types::ResponseType;
pub type LensArgs = proposal_relayer_sepolia_bindings::LensArgs;
pub fn call_args() -> (u64, u64) {
    todo!("generate CalculateArgs as args to call")
}
pub type ContractCallArgs = proposal_relayer_scroll_sepolia::ContractCallArgs;
pub fn convert(res: &CallCanisterResponse) -> ContractCallArgs {
    proposal_relayer_scroll_sepolia::convert(
        &proposal_relayer_scroll_sepolia::CallCanisterResponse {
            0: res.0.clone(),
            1: res.1.clone(),
            2: res.2.clone(),
            3: res.3.clone(),
            4: res.4.clone(),
            5: res.5.clone(),
        },
    )
}
pub fn filter(res: &CallCanisterResponse) -> bool {
    proposal_relayer_scroll_sepolia::filter(
        &proposal_relayer_scroll_sepolia::CallCanisterResponse {
            0: res.0.clone(),
            1: res.1.clone(),
            2: res.2.clone(),
            3: res.3.clone(),
            4: res.4.clone(),
            5: res.5.clone(),
        },
    )
}
