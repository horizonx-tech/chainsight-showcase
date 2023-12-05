mod types;
pub type CallCanisterResponse = types::ResponseType;
pub type CalculateArgs = relayer_spx_4500_put_bindings::CalculateArgs;
pub type LensArgs = relayer_spx_4500_put_bindings::LensArgs;

pub fn call_args() -> CalculateArgs {
    CalculateArgs { decimals: 18 }
}
pub fn filter(_: &CallCanisterResponse) -> bool {
    true
}
