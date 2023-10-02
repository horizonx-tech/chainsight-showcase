pub type LensValue = u128;

const PRECISION: u64 = 18;
const WEIGHT_BINANCE: u128 = 1;
const WEIGHT_COINGECKO: u128 = 10;
const WEIGHT_COINMARKETCAP: u128 = 2;

pub async fn calculate(targets: Vec<String>) -> LensValue {
    targets.iter().for_each(|t| ic_cdk::println!("{}", t));
    let future_1 = get_binance_usdtusd(targets.get(0).unwrap().clone());
    let future_2 = get_coingecko_usdtusd(targets.get(1).unwrap().clone());
    let future_3 = get_coinmarketcap_usdtusd(targets.get(2).unwrap().clone());
    let (result_1, result_2, result_3) = ic_web3_rs::futures::join!(future_1, future_2, future_3);
    ic_cdk::println!("{}", "price get complete");
    let quote_binance = result_1.unwrap().data.body.data.data.quote.usd;
    let price_binance = quote_binance.price;
    let quote_coingecko = result_2.unwrap().tether;
    let price_coingecko = quote_coingecko.usd;
    let quote_coinmarketcap = result_3.unwrap().data.data.quote.usd;
    let price_coinmarketcap = quote_coinmarketcap.price;

    calc_price(price_binance, price_coingecko, price_coinmarketcap)
}

fn calc_price(price_binance: f64, price_coingecko: f64, price_coinmarketcap: f64) -> u128 {
    let total_weight = WEIGHT_BINANCE + WEIGHT_COINGECKO + WEIGHT_COINMARKETCAP;
    (to_integer_price(price_binance) * WEIGHT_BINANCE
        + to_integer_price(price_coingecko) * WEIGHT_COINGECKO
        + to_integer_price(price_coinmarketcap) * WEIGHT_COINMARKETCAP)
        / total_weight
}

fn to_integer_price(price: f64) -> u128 {
    (price * (10f64.powf(PRECISION as f64))) as u128
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn test_calc_price() {
        let price_binance = 0.053008847773075414;
        let price_coingecko = 0.052895231025155566;
        let price_coinmarketcap = 0.053008847773075414;
        let p = calc_price(price_binance, price_coingecko, price_coinmarketcap);
        assert_eq!(p, 52921450274675532);
    }
}

use binance_usdtusd_bindings::SnapshotValue as SnapshotValue_binance_usdtusd;
algorithm_lens_finder!(
    "binance_usdtusd",
    "proxy_get_last_snapshot_value",
    SnapshotValue_binance_usdtusd
);
use coingecko_usdtusd_bindings::SnapshotValue as SnapshotValue_coingecko_usdtusd;
algorithm_lens_finder!(
    "coingecko_usdtusd",
    "proxy_get_last_snapshot_value",
    SnapshotValue_coingecko_usdtusd
);
use coinmarketcap_usdtusd_bindings::SnapshotValue as SnapshotValue_coinmarketcap_usdtusd;
algorithm_lens_finder!(
    "coinmarketcap_usdtusd",
    "proxy_get_last_snapshot_value",
    SnapshotValue_coinmarketcap_usdtusd
);
use chainsight_cdk::lens::LensFinder;
use chainsight_cdk_macros::algorithm_lens_finder;
async fn _get_target_proxy(target: candid::Principal) -> candid::Principal {
    let out: ic_cdk::api::call::CallResult<(candid::Principal,)> =
        ic_cdk::api::call::call(target, "get_proxy", ()).await;
    out.unwrap().0
}
