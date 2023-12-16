use lens_for_vix_float_to_uint_accessors::*;

pub type LensValue = u128;
#[derive(Clone, Debug, Default, candid :: CandidType, serde :: Deserialize, serde :: Serialize)]
pub struct CalculateArgs {
    pub num_of_digits_to_scale: Option<u32>,
}
pub async fn calculate(targets: Vec<String>, args: CalculateArgs) -> LensValue {
    let result = get_vix_in_snapshot_indexer(targets.get(0usize).unwrap().clone()).await.unwrap();
    
    convert_to_u128(result, args.num_of_digits_to_scale.unwrap_or_default())
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
