use lens_vix_spx_accessors::*;
use lens_vix_spx_bindings::near_term_options::OptionSpxInner as OptionSpxInnerType;

mod calc;
use calc::{k, options, variance, vix};

pub type LensValue = f64;
#[derive(Clone, Debug, Default, candid :: CandidType, serde :: Deserialize, serde :: Serialize)]
pub struct CalculateArgs {
    pub num_of_digits_to_scale: Option<u64>,
}
pub async fn calculate(targets: Vec<String>, args: CalculateArgs) -> LensValue {
    let near = get_near_term_options(targets.get(0usize).unwrap().clone()).await.unwrap();
    let next = get_next_term_options(targets.get(1usize).unwrap().clone()).await.unwrap();
    
    let current_nano_secs = ic_cdk::api::time() as u64;
    ic_cdk::println!("current_nano_secs: {:?}", current_nano_secs);

    // Near Term
    let near_result = near.optionChain.result.get(0usize).unwrap();
    let near_options = near_result.options.get(0usize).unwrap();
    let near_expiration_date = near_options.expirationDate;
    let near_minites_until_t = calculate_t((near_expiration_date * 1000 * 1000000) as u64, current_nano_secs);
    let near_t = near_minites_until_t / vix::N_365;
    ic_cdk::println!(">> near");
    ic_cdk::println!("expirationDate: {:?}", near_expiration_date);
    ic_cdk::println!("minites_until_t: {:?}", near_minites_until_t);
    ic_cdk::println!("t: {:?}", near_t);

    let near_strikes = &near_result.strikes;
    let near_calls = &near_options.calls;
    let near_puts = &near_options.puts;
    let near_variance = calculate_variance(near_strikes.clone(), near_calls, near_puts, near_t, 0.0); // NOTE: temp risk_free_rate

    // Next Term
    let next_result = next.optionChain.result.get(0usize).unwrap();
    let next_options = next_result.options.get(0usize).unwrap();
    let next_expiration_date = next_options.expirationDate;
    let next_minites_until_t = calculate_t( (next_expiration_date * 1000 * 1000000) as u64, current_nano_secs);
    let next_t = next_minites_until_t / vix::N_365;
    ic_cdk::println!(">> next");
    ic_cdk::println!("expirationDate: {:?}", next_expiration_date);
    ic_cdk::println!("t: {:?}", next_t);

    let next_strikes = &next_result.strikes;
    let next_calls = next_options.calls.clone().iter().map(|v|  cast_with_serde::<lens_vix_spx_bindings::next_term_options::OptionSpxInner, Box<OptionSpxInnerType>>(v)).collect::<Vec<Box<OptionSpxInnerType>>>();
    let next_puts = next_options.puts.clone().iter().map(|v| cast_with_serde::<lens_vix_spx_bindings::next_term_options::OptionSpxInner, Box<OptionSpxInnerType>>(v)).collect::<Vec<Box<OptionSpxInnerType>>>();
    let next_variance = calculate_variance(next_strikes.clone(), &next_calls, &next_puts, next_t, 0.0); // NOTE: temp risk_free_rate

    // Calculate vix
    let result = vix::calculate_vix(vix::ParamVix {
        near: vix::ParamVixPerTerm {
            variance: near_variance,
            t: near_t,
            minites_until_t: near_minites_until_t,
        },
        next: vix::ParamVixPerTerm {
            variance: next_variance,
            t: next_t,
            minites_until_t: next_minites_until_t,
        },
    });

    // Scale by args
    if let Some(scale) = args.num_of_digits_to_scale {
        let scale = 10u64.pow(scale as u32) as f64;
        return (result * scale).round();
    }
    result
}

// calculate Time to Expiration (Number of Minites)
fn calculate_t(expiration_nano_secs: u64, current_nano_secs: u64) -> f64 {
    (expiration_nano_secs - current_nano_secs) as f64 / (1000.0 * 1000000.0) / 60.0
}

fn cast_with_serde<FromT, ToT>(v: &FromT) -> ToT
where
    FromT: serde::Serialize,
    ToT: serde::de::DeserializeOwned
{
    let serialized = serde_json::to_string(v).expect("Failed to serialize by serde_json");
    serde_json::from_str(&serialized).expect("Failed to deserialize by serde_json")
}

fn calculate_variance(strikes: Vec<f64>, calls: &Vec<Box<OptionSpxInnerType>>, puts: &Vec<Box<OptionSpxInnerType>>, t: f64, r: f64) -> f64 {
    // Get ATM Strike
    let mut atm_strike = f64::MAX;
    let mut atm_strike_diff = None;
    let mut atm_strike_call = None;
    let mut atm_strike_put = None;
    for strike in strikes.clone() {
        let call = calls.iter().find(|&x| x.strike == strike);
        let put = puts.iter().find(|&x| x.strike == strike);
        if call.is_some() && put.is_some() {
            let call_unwrap = call.unwrap();
            let put_unwrap = put.unwrap();
            let call_p = (call_unwrap.bid + call_unwrap.ask) / 2.0;
            let put_p = (put_unwrap.bid + put_unwrap.ask) / 2.0;
            let result = (call_p - put_p).abs();
            if result < atm_strike {
                atm_strike = strike;
                atm_strike_diff = Some(result);
                atm_strike_call = Some(call_unwrap);
                atm_strike_put = Some(put_unwrap);
            }
        }
    }
    ic_cdk::println!("atm_strike: {:?}, diff: {:?} call: {:?}, put: {:?}", atm_strike, atm_strike_diff, atm_strike_call, atm_strike_put);

    // Calculate F, K0
    let atm_strike_call = atm_strike_call.expect("Failed to get atm_strike_call");
    let atm_strike_put = atm_strike_put.expect("Failed to get atm_strike_put");
    let f = calc::k::calculate_f(calc::k::ParamF {
        strike_price: atm_strike,
        call_price: (atm_strike_call.bid + atm_strike_call.ask) / 2.0,
        put_price: (atm_strike_put.bid + atm_strike_put.ask) / 2.0,
        risk_free_rate: r,
        time_to_expiration: t,
    });
    ic_cdk::println!("f: {:?}", f);
    let k_0_idx = k::find_closest_less_than_f(f, strikes.clone()).expect("Failed to get k_0_idx");
    let k_0 = strikes.get(k_0_idx).expect("Failed to get k_0");
    ic_cdk::println!("k_0: {:?}", k_0);

    // Select Options to calculate
    let calls_in_options = calls.iter().map(convert_call_to_option).collect::<Vec<options::Option>>();
    let puts_in_options = puts.iter().map(convert_put_to_option).collect::<Vec<options::Option>>();
    let target_calls = options::select_target_calls(*k_0, calls_in_options);
    let target_puts = options::select_target_puts(*k_0, puts_in_options);
    ic_cdk::println!("target_calls: {:?}", target_calls);
    ic_cdk::println!("target_puts: {:?}", target_puts);

    // Calculate Variance
    let (k_0_from_calls, target_calls) = target_calls.split_first().unwrap();
    let (k_0_from_puts, target_puts) = target_puts.split_first().unwrap();
    ic_cdk::println!("k_0_from_calls: {:?}", k_0_from_calls);
    ic_cdk::println!("k_0_from_puts: {:?}", k_0_from_puts);

    let around_k_0_options = if k_0_from_puts.strike_price == k_0_from_calls.strike_price {
        assert!(k_0_from_calls.strike_price == *k_0, "k_0 from calls is not equal to k_0");
        assert!(k_0_from_puts.strike_price == *k_0, "k_0 from puts is not equal to k_0");
        vec![
            convert_option_from_options_to_variance(k_0_from_puts),
            convert_option_from_options_to_variance(k_0_from_calls)
        ]
    } else {
        let k_0_option = variance::Option {
            strike_price: *k_0,
            // NOTE: calculate mid from call and ask
            bid: (k_0_from_calls.bid + k_0_from_calls.ask) / 2.0,
            ask: (k_0_from_puts.bid + k_0_from_puts.ask) / 2.0,
        };
        vec![k_0_option]
    };

    let variance_options = [
        target_puts.iter().rev().map(convert_option_from_options_to_variance).collect::<Vec<variance::Option>>(),
        around_k_0_options,
        target_calls.iter().map(convert_option_from_options_to_variance).collect::<Vec<variance::Option>>(),
    ].concat();
    
    variance::variance_per_term(variance::ParamVariance {
        options: variance_options,
        forward_price: f,
        k_0: *k_0,
        risk_free_rate: r,
        time_to_expiration: t,
    })
}

fn convert_call_to_option(v: &Box<OptionSpxInnerType>) -> options::Option {
    options::Option {
        strike_price: v.strike,
        bid: v.bid,
        ask: v.ask,
        is_call: true,
    }
}

fn convert_put_to_option(v: &Box<OptionSpxInnerType>) -> options::Option {
    options::Option {
        strike_price: v.strike,
        bid: v.bid,
        ask: v.ask,
        is_call: false,
    }
}

fn convert_option_from_options_to_variance(v: &options::Option) -> variance::Option {
    variance::Option {
        strike_price: v.strike_price,
        bid: v.bid,
        ask: v.ask,
    }
}