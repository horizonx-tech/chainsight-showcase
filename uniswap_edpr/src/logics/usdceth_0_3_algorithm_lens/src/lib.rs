use usdceth_0_3_algorithm_lens_accessors :: * ;
use std :: collections :: HashMap;
use serde_json :: Value;

# [derive (Clone , Debug , Default , candid :: CandidType , serde :: Deserialize , serde :: Serialize)]
pub struct LensValue {
    pub address: String,
    pub current_price: f32,
    pub range_top: f32,
    pub range_bottom: f32,
    pub edpr: f32
}

# [derive (Default, serde :: Deserialize)]
pub struct Tick {
    fee_growth_outside_0x128: u128,
    fee_growth_outside_1x128: u128,
    index: i32,
    initialized: bool,
    liquidity_gross: u128,
    liquidity_net: i128
}

pub async fn calculate (targets : Vec < String >) -> LensValue {
    let pool_fees_result = get_get_last_snapshot_value_in_usdceth_0_3_pool_fees (targets . get (0usize) . unwrap () . clone ()) . await ;
    let tc28x6_result = get_get_last_snapshot_value_in_usdceth_0_3_tcumul_28x6hr (targets . get (1usize) . unwrap () . clone ()) . await ;
    let tc30_result = get_get_last_snapshot_value_in_usdceth_0_3_tcumul_30_min (targets . get (2usize) . unwrap () . clone ()) . await ;
    let v3pool_result = get_get_last_snapshot_value_in_usdceth_0_3_v3pool (targets . get (3usize) . unwrap () . clone ()) . await ;
    let pool_usd_prices_result = get_get_last_snapshot_value_in_pool_usd_prices (targets . get (4usize) . unwrap () . clone ()) . await ;
    
    let fees_24h_usd = pool_fees_result.unwrap().result[0].fees_24h_usd;

    let t0_price_usd  = pool_usd_prices_result.clone().unwrap().result.t0_price_usd;
    let t1_price_usd  = pool_usd_prices_result.clone().unwrap().result.t1_price_usd;

    let address = v3pool_result.clone().unwrap().result.address;
    let current_tick_liquidity = v3pool_result.clone().unwrap().result.liquidity;
    let sqrt_ratio_x96 = v3pool_result.clone().unwrap().result.sqrt_ratio_x96;
    let tick_current = v3pool_result.clone().unwrap().result.tick_current;
    let tick_spacing = v3pool_result.clone().unwrap().result.tick_spacing;
    let ticks_str = v3pool_result.clone().unwrap().result.ticks;
    let ticks = serde_json::from_str::<HashMap<String, Value>>(&ticks_str);

    let tick_cumul_28x6h = tc28x6_result.unwrap().0;
    let tick_cumul_30m = tc30_result.unwrap().0;

    let price_x96 = u128::pow(sqrt_ratio_x96.into(), 2) as f32;
    let mut current_price = price_x96 / f32::powf(2.0,192.0);
    let current_price = current_price / f32::powf(10.0,12.0);

    let mut compressed: i32 = tick_current / tick_spacing;
    if tick_current <0 && tick_current % tick_spacing != 0 {
        compressed -= 1;
    }
    let floor = compressed * tick_spacing;
    let mut min_tick = floor;
    let mut max_tick = floor + tick_spacing;
    for i in (1..29).rev() {
        let new_tick = (tick_cumul_28x6h[i] as i32 - tick_cumul_28x6h[i-1] as i32) / 21600;
        if new_tick < min_tick {
            min_tick = new_tick;
        } else if new_tick > max_tick {
            max_tick = new_tick;
        }
    }
    let mut state = current_tick_liquidity as i128;
    let mut sum_liquidity: i128 = 0;
    if tick_current >= min_tick && tick_current < max_tick {
        sum_liquidity = current_tick_liquidity as i128;
    }

    let ticks_clone = ticks.expect("reason").clone();
    for i in (floor + tick_spacing..max_tick).step_by(tick_spacing as usize) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let tick_val = ticks_clone.get(&i.to_string());
            let tick: Tick = serde_json::from_value(tick_val.unwrap().clone()).unwrap();
            let liquidity_net_ = tick.liquidity_net;
            state += liquidity_net_;
            sum_liquidity += state;
        })) {
            Ok(_) => {}
            Err(_) => {}
        }
    }
    state = current_tick_liquidity as i128;
    for i in (floor + tick_spacing..min_tick).rev().step_by(tick_spacing as usize) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let tick_val = ticks_clone.get(&i.to_string());
            let tick: Tick = serde_json::from_value(tick_val.unwrap().clone()).unwrap();
            let liquidity_net_ = tick.liquidity_net;
            state -= liquidity_net_;
            sum_liquidity += state;
        })) {
            Ok(_) => {}
            Err(_) => {}
        }
    }
    let upper_sqrt_price_x96 = f32::powf(1.0001, max_tick as f32);
    let mut range_top = upper_sqrt_price_x96 / f32::powf(2.0,192.0);
    range_top = range_top / f32::powf(10.0,12.0);
    let lower_sqrt_price_x96 = f32::powf(1.0001, max_tick as f32);
    let mut range_bottom = upper_sqrt_price_x96 / f32::powf(2.0,192.0);
    range_bottom = range_bottom / f32::powf(10.0,12.0);
    
    let tvl_in_range = sum_liquidity as f32
        * ((sqrt_ratio_x96 as f32 / upper_sqrt_price_x96)
            * (upper_sqrt_price_x96 - sqrt_ratio_x96 as f32)
            + (sqrt_ratio_x96 as f32 - lower_sqrt_price_x96));

    let fees_24h_eth = fees_24h_usd / t1_price_usd;
    let edr = fees_24h_eth / tvl_in_range;
    let edpr = edr / 100.0;

    let result = LensValue {
        address,
        current_price,
        range_top,
        range_bottom,
        edpr
    };
    result
}