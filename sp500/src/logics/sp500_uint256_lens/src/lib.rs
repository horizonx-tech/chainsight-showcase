use std::ops::Mul;

use chainsight_cdk::core::U256;
use sp500_uint256_lens_accessors::*;
pub type LensValue = U256;

const DECIMALS: u32 = 8;
pub async fn calculate(targets: Vec<String>) -> LensValue {
    let result =
        get_get_last_snapshot_value_in_sp500_indexer_https(targets.get(0usize).unwrap().clone())
            .await;
    let market_price = result.unwrap().chart.result[0].meta.regularMarketPrice;
    convert_to_u256(market_price)
}

fn convert_to_u256(value: f64) -> LensValue {
    let v = value.mul(10.0f64.powf(DECIMALS as f64)) as u128;
    U256::from(ic_web3_rs::types::U256::from(v))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_convert_to_u256() {
        let v = convert_to_u256(4559.34);
        assert_eq!(
            v,
            U256::from(ic_web3_rs::types::U256::from(455934000000_u128))
        );
    }
}
