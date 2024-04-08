use std::collections::BTreeMap;

use ic_web3_rs::futures;
use realized_volatility_collector_accessors::*;
use realized_volatility_collector_bindings::realized_volatility_from_u256;

pub type LensValue = BTreeMap<String, Result<f64, String>>;
pub type CalculateArgs = Vec<String>;

const COUNT: u64 = 24; // Hourly

pub async fn calculate(targets: Vec<String>, args: CalculateArgs) -> LensValue {
    let calculator_id = targets.get(0).unwrap();
    let results = futures::future::join_all(
        args
            .clone()
            .iter()
            .map(|target| get_rvol(calculator_id.clone(), realized_volatility_from_u256::LensArgs {
                args: realized_volatility_from_u256::CalculateArgs { snapshot_count: COUNT },
                targets: vec![target.clone()],
            })),
    ).await;
    
    let res_vec = args.iter().enumerate().map(|(i, target)| {
        let result = results.get(i).unwrap();
        (target.clone(), result.clone())
    }).collect::<Vec<(String, Result<f64, String>)>>();
    res_vec.into_iter().collect()
}
