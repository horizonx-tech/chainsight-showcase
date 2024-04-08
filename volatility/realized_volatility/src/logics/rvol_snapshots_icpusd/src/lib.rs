pub type CalculateArgs = rvol_snapshots_icpusd_bindings::CalculateArgs;
pub type LensArgs = rvol_snapshots_icpusd_bindings::LensArgs;
pub fn call_args() -> CalculateArgs {
    CalculateArgs { snapshot_count: 24 }
}
