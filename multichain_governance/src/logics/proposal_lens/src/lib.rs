use proposal_lens_accessors::*;

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
    let results = get_events_from_to_in_proposal_factory_event_indexer_sepolia(
        targets.get(0usize).unwrap().clone(),
        args,
    )
    .await
    .unwrap();
    results
        .iter()
        .map(|r| {
            let block = r.0;
            let events = &r.1;
            let e: Vec<(u128, String, u128, u128, u128, u64)> = events
                .iter()
                .map(|e| {
                    (
                        e.id.value.parse().unwrap(),
                        e.proposer.clone(),
                        e.startTimestamp.value.parse().unwrap(),
                        e.endTimestamp.value.parse().unwrap(),
                        e.chainId.value.parse().unwrap(),
                        block,
                    )
                })
                .collect();
            e
        })
        .flatten()
        .map(|e| (e.0, e.1, e.2, e.3, e.4, e.5))
        .fold(
            (vec![], vec![], vec![], vec![], vec![], vec![]),
            |mut acc, e| {
                acc.0.push(e.0);
                acc.1.push(e.1);
                acc.2.push(e.2);
                acc.3.push(e.3);
                acc.4.push(e.4);
                acc.5.push(e.5.into());
                acc
            },
        )
}
