use lens_oasusd_accessors::*;
pub type LensValue = u128;
const PRECISION: u128 = 18;
const WEIGHT_COINGECKO: u128 = 10;
const WEIGHT_COINMARKETCAP: u128 = 10;
pub async fn calculate(targets: Vec<String>) -> LensValue {
    let future_1 = get_coingecko_oasusd(targets.get(0usize).unwrap().clone());
    let future_2 = get_coinmarketcap_oasusd(targets.get(1usize).unwrap().clone());
    let (result_1, result_2) = ic_web3_rs::futures::join!(future_1, future_2);
    let quote_coingecko = result_1.unwrap().oasys;
    let price_coingecko = quote_coingecko.usd;
    let quote_coinmarketcap = result_2.unwrap().data.data.quote.usd;
    let price_coinmarketcap = quote_coinmarketcap.price;
    calc_price(price_coingecko, price_coinmarketcap)
}
fn calc_price(price_coingecko: f64, price_coinmarketcap: f64) -> u128 {
    let total_weight = WEIGHT_COINGECKO + WEIGHT_COINMARKETCAP;
    (to_int_price(price_coingecko) * WEIGHT_COINGECKO
        + to_int_price(price_coinmarketcap) * WEIGHT_COINMARKETCAP)
        / total_weight
}
fn to_int_price(price: f64) -> u128 {
    (price * (10f64.powf(PRECISION as f64))) as u128
}