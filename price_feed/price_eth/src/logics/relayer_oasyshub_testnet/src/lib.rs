mod types;
pub type CallCanisterResponse = types::ResponseType;
pub type CallCanisterArgs = ();
pub fn call_args() -> CallCanisterArgs {
    ()
}
pub fn filter(_: &CallCanisterResponse) -> bool {
    true
}
