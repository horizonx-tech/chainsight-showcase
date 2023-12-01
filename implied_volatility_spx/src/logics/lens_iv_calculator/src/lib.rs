use lens_iv_calculator_accessors::*;
use lens_iv_calculator_bindings::option::Meta;

mod calculator;
mod time;

pub type LensValue = f64;
#[derive(Clone, Debug, Default, candid :: CandidType, serde :: Deserialize, serde :: Serialize)]
pub struct CalculateArgs {
    pub initial_sigma: f64,
    pub tolerance: f64,
    pub attempt_count: u64,
    pub num_of_digits_to_scale: Option<u64>,
}
pub async fn calculate(targets: Vec<String>, args: CalculateArgs) -> LensValue {
    ic_cdk::println!("args: {:?}", &args);
    let target_for_underlying = targets.get(0usize).unwrap().to_owned();
    let target_for_option = targets.get(1usize).unwrap().to_owned();
    
    let underlying_current_price = get_underlying_chart(target_for_underlying)
        .await.unwrap_or_else(|msg| panic!("{}", msg))
        .chart.result.get(0).expect("no result in underlying's chart").meta.regularMarketPrice;
    ic_cdk::println!("input: underlying={}", underlying_current_price);

    let option_meta = get_option_chart(target_for_option)
        .await.unwrap_or_else(|msg| panic!("{}", msg))
        .chart.result.get(0).expect("no result in option's chart").meta.clone();
    ic_cdk::println!("input: option={:?}", &option_meta);

    let param = generate_seek_iv_param(underlying_current_price, option_meta, args.clone());
    ic_cdk::println!("param: {:?}", &param);
    let (sigma, attempt) = calculator::seek_implied_volatility(param);
    ic_cdk::println!("output: sigma={}, attempt={}", sigma, attempt);

    // consider the scale of args
    if let Some(scale) = args.num_of_digits_to_scale {
        let scale = 10u64.pow(scale as u32) as f64;
        return (sigma * scale).round();
    }

    sigma
}

fn generate_seek_iv_param(
    underlying_current_price: f64,
    option_meta: Meta,
    args: CalculateArgs,
) -> calculator::SeekIvParam {
    let CalculateArgs { initial_sigma, tolerance, attempt_count , num_of_digits_to_scale: _} = args;
    
    let current_ts_sec = ic_cdk::api::time() / (1000 * 1000000); // nanosec -> sec
    let (k, t, is_call) = param_from_symbol(option_meta.symbol, current_ts_sec as f64);

    calculator::SeekIvParam {
        s: underlying_current_price,
        k,
        t,
        r: 0.0,
        is_call, // NOTE: assuming index options
        market_price: option_meta.regularMarketPrice,
        initial_sigma,
        tolerance,
        attempt_count,
    }
}

fn param_from_symbol(s: String, current_sec: f64) -> (f64, f64, bool) {
    assert!(s.len() == 18);
    let expiry_yymmdd = &s[3..9];
    let option_type = s.chars().nth(9).unwrap();
    let strike_price = &s[10..];
    ic_cdk::println!("param_from_symbol: expiry_yymmdd={}, option_type={}, strike_price={}", expiry_yymmdd, option_type, strike_price);

    let k = strike_price.parse::<f64>().unwrap() / 1000.0;

    let t = {
        let yyyy = expiry_yymmdd.get(0..2).unwrap().parse::<u32>().unwrap() + 2000;
        let mm = expiry_yymmdd.get(2..4).unwrap().parse::<u32>().unwrap();
        let dd = expiry_yymmdd.get(4..6).unwrap().parse::<u32>().unwrap();
    
        let expiry_ts_sec = time::date_to_epoch(
            yyyy, mm, dd
        ) as f64;

        assert!(expiry_ts_sec >= current_sec);
        (expiry_ts_sec - current_sec) / (365.0 * 24.0 * 60.0 * 60.0)
    };

    let is_call = match option_type {
        'C' => true,
        'P' => false,
        _ => panic!("invalid option type"),
    };

    (k, t, is_call)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_param_from_symbol() {
        let epoch = 1709251200.0; // 20240315
        let (k, t, is_call) = param_from_symbol("SPX240315C04500000".to_owned(), epoch);
        assert_eq!(k, 4500.0);
        assert_eq!(t, 0.038356164383561646);
        assert_eq!(is_call, true);

        let (k, t, is_call) = param_from_symbol("SPX240315P04500000".to_owned(), epoch);
        assert_eq!(k, 4500.0);
        assert_eq!(t, 0.038356164383561646);
        assert_eq!(is_call, false);
    }
}