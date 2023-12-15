mod types;
pub type CallCanisterResponse = types::ResponseType;
pub type CalculateArgs = relayer_spx_vix_bindings::CalculateArgs;
pub type LensArgs = relayer_spx_vix_bindings::LensArgs;
pub fn call_args() -> CalculateArgs {
    CalculateArgs {
        num_of_digits_to_scale: Some(18),
    }
}
pub fn filter(_: &CallCanisterResponse) -> bool {
    true
}
