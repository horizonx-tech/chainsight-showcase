pub type CalculateArgs = snapshots_iv_for_spx_4500_put_bindings::CalculateArgs;
pub type LensArgs = snapshots_iv_for_spx_4500_put_bindings::LensArgs;

#[ic_cdk::query] // temp
#[candid::candid_method(query)] // temp
pub fn call_args() -> CalculateArgs {
    CalculateArgs {
        initial_sigma: 0.2,
        tolerance: 0.000001,
        attempt_count: 100,
        num_of_digits_to_scale: None,
    }
}