use lens_usdcusd_accessors::*;
const PRECISION: u32 = 18;
pub type LensValue = u128;

pub async fn calculate(targets: Vec<String>) -> LensValue {
    let usdcusd = get_chainlink_usdcusd(targets.get(0usize).unwrap().clone())
        .await
        .unwrap()
        .parse::<u128>()
        .unwrap();
    format_usdcusd(usdcusd)
}

fn format_usdcusd(usdcusd: u128) -> u128 {
    usdcusd * 10u128.pow(PRECISION - 8)
}