use lens_wbtcusd_accessors::*;
pub type LensValue = u128;
pub async fn calculate(targets: Vec<String>) -> LensValue {
    let btcusd = get_chainlink_btcusd(targets.get(0).unwrap().clone())
        .await
        .unwrap()
        .parse::<u128>()
        .unwrap();
    let wbtcbtc = get_chainlink_wbtcbtc(targets.get(1).unwrap().clone())
        .await
        .unwrap()
        .parse::<u128>()
        .unwrap();
    let wbtcusd = calc_wbtcusd(btcusd, wbtcbtc);
    wbtcusd
}
fn calc_wbtcusd(btcusd: u128, wbtcbtc: u128) -> u128 {
    btcusd * wbtcbtc / 100000000
}
#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn test_calc_wbtcusd() {
        let btcusd = 2956575400000;
        let wbtcbtc = 100200000;
        let wbtcusd = calc_wbtcusd(btcusd, wbtcbtc);
        assert_eq!(wbtcusd, 2962488550800);
    }
}
