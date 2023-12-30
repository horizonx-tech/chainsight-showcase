use proposal_lens_accessors::*;
use proposal_lens_bindings::proposal_factory_event_indexer_sepolia::RequestArgsType;

pub type LensValue = (
    Vec<u128>,
    Vec<String>,
    Vec<u128>,
    Vec<u128>,
    Vec<u128>,
    Vec<u128>,
);
pub type CalculateArgs = (u64, u64);

pub async fn calculate(targets: Vec<String>, args: CalculateArgs) -> LensValue {
    let _result = get_events_from_to_in_proposal_factory_event_indexer_sepolia(
        targets.get(0usize).unwrap().clone(),
        RequestArgsType {
            0: args.0,
            1: args.1,
        },
    )
    .await;
    todo!()
}
