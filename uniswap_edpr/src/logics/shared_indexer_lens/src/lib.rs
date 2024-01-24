use shared_indexer_lens_accessors::*;
pub type LensValue = u64;

const DECIMALS: u32 = 18;

pub async fn calculate(targets: Vec<String>) -> LensValue {
    let result = get_get_last_snapshot_value_in_indexer(targets.get(0usize).unwrap().clone()).await;
    convert_to_u64(result.unwrap().edpr, DECIMALS)
}

fn convert_to_u64(value: f32, dec: u32) -> u64 {
    let v = value * (10u128.pow(dec) as f32);
    v.round() as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_convert_to_u64() {
        let v = convert_to_u64(4559.00, 4);
        assert_eq!(v, 45590000u64);
    }
}
