mod types;
use std::{collections::HashMap, str::FromStr};

use candid::Principal;
use types::Snapshot;
pub type CalculateArgs = Vec<String>;
use crate::types::BulkSnapshotIndexerHttps;
#[derive(Clone, Debug, Default, candid :: CandidType, serde :: Deserialize, serde :: Serialize)]
pub struct LensValue {
    pub value: f64,
}

pub async fn calculate(targets: Vec<String>, args: CalculateArgs) -> LensValue {
    let target = Principal::from_str(&targets[0]).unwrap();
    let indexer = BulkSnapshotIndexerHttps::new(target);
    let results = indexer.batch_get(args).await.unwrap();
    LensValue {
        value: calc_score(results),
    }
}

fn calc_score(source: HashMap<String, Option<Snapshot>>) -> f64 {
    let mut sum = 0.0;
    let mut count = 0;
    for (_, snapshot) in source {
        if let Some(snapshot) = snapshot {
            let value = snapshot.value.raw;
            let value: f64 = bincode::deserialize(&value).unwrap();
            sum += value;
            count += 1;
        }
    }
    sum / count as f64
}
