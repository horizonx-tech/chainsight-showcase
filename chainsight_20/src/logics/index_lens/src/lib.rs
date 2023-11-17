use ic_web3_rs::futures::future;
use index_lens_accessors::*;
use index_lens_bindings::ada_indexer_coingecko::SnapshotValue;
const DIVISOR: f64 = 10000000.0;
const DECIMALS: i32 = 6;
#[derive(Clone, Debug, Default, candid :: CandidType, serde :: Deserialize, serde :: Serialize)]
pub struct LensValue {
    pub index: f64,
}
pub async fn calculate(targets: Vec<String>) -> LensValue {
    let call_results = aggregate_get(targets).await;

    LensValue {
        index: calculate_index_value(call_results),
    }
}

async fn aggregate_get(targets: Vec<String>) -> Vec<Box<SnapshotValue>> {
    let promices: Vec<_> = targets
        .iter()
        .map(|t| return get_get_last_snapshot_value_in_ada_indexer_coingecko(t.to_string()))
        .collect();
    let results = future::join_all(promices).await;
    results.iter().map(|r| return r.clone().unwrap()).collect()
}

fn calculate_index_value(snapshots: Vec<Box<SnapshotValue>>) -> f64 {
    let tokens = snapshots.len();
    let mut total_market_cap_with_iwf: f64 = 0.0;
    snapshots.iter().for_each(|s| {
        let market_cap = s.market_data.market_cap.usd;
        let circulating_supply = s.market_data.circulating_supply;
        let total_supply = s.market_data.total_supply;
        let iwf = circulating_supply / total_supply;
        let market_cap_with_iwf = market_cap as f64 * iwf;
        total_market_cap_with_iwf += market_cap_with_iwf;
    });
    let v = total_market_cap_with_iwf / tokens as f64 / DIVISOR;
    (v * 10_f64.powi(DECIMALS)).round() / 10_f64.powi(DECIMALS)
}

#[cfg(test)]
mod tests {
    use index_lens_bindings::ada_indexer_coingecko::{MarketCap, MarketData, SnapshotValue};

    use crate::calculate_index_value;

    #[test]
    fn test_calculate_index_value() {
        let tokens = vec![
            SnapshotValue {
                market_data: MarketData {
                    circulating_supply: 19543531.0,
                    market_cap: MarketCap { usd: 731891164608 },
                    total_supply: 21000000.0,
                },
            },
            SnapshotValue {
                market_data: MarketData {
                    circulating_supply: 120254695.0,
                    market_cap: MarketCap { usd: 247012763906 },
                    total_supply: 120254695.0,
                },
            },
        ];
        let result = calculate_index_value(tokens.into_iter().map(|t| Box::new(t)).collect());
        assert_eq!(result, 46407.156443);
    }
}
