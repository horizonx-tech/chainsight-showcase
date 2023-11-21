use index_lens_accessors::*;
use index_lens_bindings::market_indexer_coingecko::Snapshot;
const DIVISOR: f64 = 10000000.0;
const DECIMALS: i32 = 6;
#[derive(Clone, Debug, Default, candid :: CandidType, serde :: Deserialize, serde :: Serialize)]
pub struct LensValue {
    pub index: f64,
}
pub async fn calculate(targets: Vec<String>) -> LensValue {
    let result =
        get_get_last_snapshot_in_market_indexer_coingecko(targets.get(0usize).unwrap().clone())
            .await
            .unwrap();

    ic_cdk::println!("result!!!!: {:?}", calculate_index_value(result.clone()));
    LensValue {
        index: calculate_index_value(result),
    }
}

fn calculate_index_value(snapshot: Box<Snapshot>) -> f64 {
    let tokens = snapshot.value.len();
    let mut total_market_cap_with_iwf: f64 = 0.0;
    snapshot.value.iter().for_each(|s| {
        let market_cap = s.market_cap as f64;
        let circulating_supply = s.circulating_supply;
        let total_supply = s.total_supply;
        let iwf = circulating_supply / total_supply;
        let market_cap_with_iwf = market_cap as f64 * iwf;
        total_market_cap_with_iwf += market_cap_with_iwf;
    });
    let v = total_market_cap_with_iwf / tokens as f64 / DIVISOR;
    (v * 10_f64.powi(DECIMALS)).round() / 10_f64.powi(DECIMALS)
}

#[cfg(test)]
mod tests {

    use index_lens_bindings::market_indexer_coingecko::{Snapshot, SnapshotValue2};

    use crate::calculate_index_value;

    #[test]
    fn test_calculate_index_value() {
        let tokens = vec![
            SnapshotValue2 {
                market_cap: 731891164608,
                circulating_supply: 19543531.0,
                total_supply: 21000000.0,
            },
            SnapshotValue2 {
                market_cap: 247012763906,
                circulating_supply: 120254695.0,
                total_supply: 120254695.0,
            },
        ];
        let result = calculate_index_value(Box::new(Snapshot {
            timestamp: 0,
            value: tokens.into_iter().map(|t| Box::new(t)).collect(),
        }));
        assert_eq!(result, 46407.156443);
    }
}
