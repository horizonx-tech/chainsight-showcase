use voting_lens_accessors::*;

pub type LensValue = (Vec<u128>, Vec<String>, Vec<bool>, Vec<u128>, Vec<u128>);
pub type CalculateArgs = (u64, u64);

pub async fn calculate(targets: Vec<String>, args: CalculateArgs) -> LensValue {
    let _result = get_events_from_to_in_voting_event_indexer_sepolia(
        targets.get(0usize).unwrap().clone(),
        todo!("Arguments to be used in this call"),
    )
    .await;
    todo!()
}
