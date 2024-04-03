pub type CalculateArgs = realized_volatility_snapshots_ethusd_bindings::CalculateArgs;
pub type LensArgs = realized_volatility_snapshots_ethusd_bindings::LensArgs;
pub fn call_args() -> CalculateArgs {
    CalculateArgs {
        snapshot_count: 24,
    }
}
