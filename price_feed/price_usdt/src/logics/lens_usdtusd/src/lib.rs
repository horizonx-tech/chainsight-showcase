use lens_usdtusd_accessors::*;
const PRECISION: u32 = 18;
pub type LensValue = u128;

pub async fn calculate(targets: Vec<String>) -> LensValue {
    let usdtusd = get_chainlink_usdtusd(targets.get(0usize).unwrap().clone())
        .await
        .unwrap()
        .parse::<u128>()
        .unwrap();
    format_usdtusd(usdtusd)
}

fn format_usdtusd(usdtusd: u128) -> u128 {
    usdtusd * 10u128.pow(PRECISION - 8)
}