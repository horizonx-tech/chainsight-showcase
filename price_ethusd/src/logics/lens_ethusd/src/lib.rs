use lens_ethusd_accessors::*;
const PRECISION: u32 = 18;
pub type LensValue = u128;
pub async fn calculate(targets: Vec<String>) -> LensValue {
    let ethusd = get_chainlink_ethusd(targets.get(0usize).unwrap().clone())
        .await
        .unwrap()
        .parse::<u128>()
        .unwrap();
    format_ethusd(ethusd)
}

// The raw data is 8 digits precision, so we need to convert it into 18 digits
fn format_ethusd(ethusd: u128) -> u128 {
    ethusd * 10u128.pow(PRECISION - 8)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn test_format_ethusd() {
        let ethusd = 2956575400000;
        let formated = format_ethusd(ethusd);
        assert_eq!(formated, 29565754000000000000000);
    }
}
