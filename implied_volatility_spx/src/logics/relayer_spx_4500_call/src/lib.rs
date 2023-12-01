mod types;
pub type CallCanisterResponse = types::ResponseType;
pub type CalculateArgs = relayer_spx_4500_call_bindings::CalculateArgs;
pub type LensArgs = relayer_spx_4500_call_bindings::LensArgs;
pub fn call_args() -> CalculateArgs {
    CalculateArgs {
        initial_sigma: 0.2,
        tolerance: 0.000001,
        attempt_count: 100,
        num_of_digits_to_scale: Some(18),
    }
}
pub fn filter(_: &CallCanisterResponse) -> bool {
    true
}
