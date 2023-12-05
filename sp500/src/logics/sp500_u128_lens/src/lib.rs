use sp500_u128_lens_accessors::*;
pub type LensValue = u128;
const DECIMALS: u32 = 8;
pub async fn calculate(targets: Vec<String>) -> LensValue {
    let result =
        get_get_last_snapshot_value_in_sp500_indexer_https(targets.get(0usize).unwrap().clone())
            .await;
    let market_price = result.unwrap().chart.result[0].meta.regularMarketPrice;
    convert_to_u128(market_price)
}

fn convert_to_u128(value: f64) -> u128 {
    let v = value * (10u128.pow(DECIMALS) as f64);
    v.round() as u128
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_convert_to_u128() {
        let v = convert_to_u128(4559.34);
        assert_eq!(v, 455934000000_u128);
    }
}
