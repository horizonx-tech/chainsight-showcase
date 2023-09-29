use chainlink_ethusd_bindings::get_chainlink_ethusd;
const PRECISION: u32 = 18;
pub async fn calculate(targets: Vec<String>) -> u128 {
    let result = get_chainlink_ethusd(targets.get(0).unwrap().clone())
        .await
        .unwrap();
    let raw = result.parse::<u128>().unwrap();
    // raw data is 8 digits and convert it into 18 digits
    raw * 10u128.pow(PRECISION - 8)
}
