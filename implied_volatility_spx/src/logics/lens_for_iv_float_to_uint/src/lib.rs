use lens_for_iv_float_to_uint_accessors::*;

pub type LensValue = u128;
pub const DECIMALS: u32 = 18;
pub async fn calculate(targets: Vec<String>) -> LensValue {
    let result = get_iv_in_snapshot_indexer(targets.get(0usize).unwrap().clone())
        .await.unwrap_or_else(|msg| panic!("{}", msg));
    convert_to_u128(result, DECIMALS)
}

fn convert_to_u128(value: f64, dec: u32) -> u128 {
    let v = value * (10u128.pow(dec) as f64);
    v.round() as u128
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_convert_to_u128() {
        let v = convert_to_u128(4559.34, 8);
        assert_eq!(v, 455934000000_u128);
    }
}
