use shared_algorithm_lens_accessors :: * ;

# [derive (Clone , Debug , Default , candid :: CandidType , serde :: Deserialize , serde :: Serialize)]
pub struct LensValue {
  pub address: String,
  pub current_price: f32,
  pub range_top: f32,
  pub range_bottom: f32,
  pub edpr: f32
}

pub async fn calculate (targets : Vec < String >) -> LensValue {
  let pool_fees_result = get_get_last_snapshot_value_in_pool_fees (targets . get (0usize) . unwrap () . clone ()) . await ;
  let tc28x6_result = get_get_last_snapshot_value_in_tcumul_28x6hr (targets . get (1usize) . unwrap () . clone ()) . await ;
  let v3pool_result = get_get_last_snapshot_value_in_v3pool (targets . get (2usize) . unwrap () . clone ()) . await ;
  let eth_usdc_price_result = get_get_last_snapshot_value_in_eth_usdc_price (targets . get (3usize) . unwrap () . clone ()) . await ;
  
  let fees_24h_usd = pool_fees_result.unwrap().result[0].fees_24h_usd;

  let eth_price_usd  = eth_usdc_price_result.clone().unwrap().result.t1_price_usd;

  let address = v3pool_result.clone().unwrap().result.address;
  let current_tick_liquidity_str = v3pool_result.clone().unwrap().result.liquidity;
  let current_tick_liquidity = u32::from_str_radix(current_tick_liquidity_str.trim_start_matches("0x"), 16).unwrap();
  let sqrt_ratio_x96_str = v3pool_result.clone().unwrap().result.sqrt_ratio_x96;
  let sqrt_ratio_x96 = u32::from_str_radix(sqrt_ratio_x96_str.trim_start_matches("0x"), 16).unwrap();
  let tick_current = v3pool_result.clone().unwrap().result.tick_current;
  let tick_spacing = v3pool_result.clone().unwrap().result.tick_spacing;
  let token0 = v3pool_result.clone().unwrap().result.token0;

  let tick_cumul_28x6h = tc28x6_result.unwrap().0;

  let price_x96 = u128::pow(sqrt_ratio_x96.into(), 2) as f32;
  let mut current_price = price_x96 / f32::powf(2.0,192.0);
  current_price = current_price / f32::powf(10.0,12.0);

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
  
  let ticks_in_range = (max_tick - min_tick) / tick_spacing;
  let sum_liquidity: i128 = current_tick_liquidity as i128 * ticks_in_range as i128;
  
  let upper_sqrt_price_x96 = f32::powf(1.0001, max_tick as f32);
  let mut range_top = upper_sqrt_price_x96 / f32::powf(2.0,192.0);
  range_top = range_top / f32::powf(10.0,12.0);
  let lower_sqrt_price_x96 = f32::powf(1.0001, max_tick as f32);
  let mut range_bottom = upper_sqrt_price_x96 / f32::powf(2.0,192.0);
  range_bottom = range_bottom / f32::powf(10.0,12.0);
  
  let est_tvl_in_range = sum_liquidity as f32
    * ((sqrt_ratio_x96 as f32 / upper_sqrt_price_x96)
      * (upper_sqrt_price_x96 - sqrt_ratio_x96 as f32)
      + (sqrt_ratio_x96 as f32 - lower_sqrt_price_x96));

  let fees_24h_eth = fees_24h_usd / eth_price_usd;
  let edr = fees_24h_eth / est_tvl_in_range;
  let edpr = edr * 100.0;

  if token0 == "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2" {
    current_price = 1.0 / current_price;
    let new_range_top = 1.0 / range_bottom;
    let new_range_bottom = 1.0 / range_top;
    range_top = new_range_top;
    range_bottom = new_range_bottom;
  }

  let result = LensValue {
    address,
    current_price,
    range_top,
    range_bottom,
    edpr
  };
  result
}
