pub type CalculateArgs = rvol_snapshots_arbusd_bindings::CalculateArgs;
pub type LensArgs = rvol_snapshots_arbusd_bindings::LensArgs;
pub fn call_args() -> CalculateArgs {
    CalculateArgs { snapshot_count: 24 }
}
