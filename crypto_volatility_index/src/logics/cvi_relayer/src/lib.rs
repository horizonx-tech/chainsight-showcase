mod types;
pub type CallCanisterResponse = types::ResponseType;
pub type CalculateArgs = cvi_relayer_bindings::CalculateArgs;
pub type LensArgs = cvi_relayer_bindings::LensArgs;
pub fn call_args() -> CalculateArgs {
    CalculateArgs {
        num_of_digits_to_scale: None,
    }
}
pub fn filter(_: &CallCanisterResponse) -> bool {
    true
}
