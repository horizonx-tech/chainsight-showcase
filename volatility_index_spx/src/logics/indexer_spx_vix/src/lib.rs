pub type CalculateArgs = indexer_spx_vix_bindings::CalculateArgs;
pub type LensArgs = indexer_spx_vix_bindings::LensArgs;
pub fn call_args() -> CalculateArgs {
    CalculateArgs {
        num_of_digits_to_scale: None,
    }
}
