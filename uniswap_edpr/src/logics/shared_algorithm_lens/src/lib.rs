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
  
  let fees_24h_usd = pool_fees_result.clone().unwrap().result[0].fees_24h_usd;
  let t0_decimals = pool_fees_result.clone().unwrap().result[0].pool_summary_level_1.t0_decimals;
  let t1_decimals =  pool_fees_result.clone().unwrap().result[0].pool_summary_level_1.t1_decimals;

  let eth_price_usd  = eth_usdc_price_result.clone().unwrap().result.t1_price_usd;

  let address = v3pool_result.clone().unwrap().result.address;
  let current_tick_liquidity_str = v3pool_result.clone().unwrap().result.liquidity;
  let current_tick_liquidity = i128::from_str_radix(current_tick_liquidity_str.trim_start_matches("0x"), 16).unwrap();
  let sqrt_ratio_x96_str = v3pool_result.clone().unwrap().result.sqrt_ratio_x96;
  let sqrt_ratio_x96 = u128::from_str_radix(sqrt_ratio_x96_str.trim_start_matches("0x"), 16).unwrap();
  let tick_current = v3pool_result.clone().unwrap().result.tick_current;
  let tick_spacing = v3pool_result.clone().unwrap().result.tick_spacing;
  let token0 = v3pool_result.clone().unwrap().result.token0;

  let tick_cumul_28x6h = tc28x6_result.unwrap().0;

  let current_sqrt_price = sqrt_ratio_x96 as f32 / f32::powf(2.0,96.0);
  let mut current_price = current_sqrt_price * current_sqrt_price;

  let mut compressed: i32 = tick_current / tick_spacing;
  if tick_current <0 && tick_current % tick_spacing != 0 {
    compressed -= 1;
  }
  let floor = compressed * tick_spacing;
  let mut min_tick = floor;
  let mut max_tick = floor + tick_spacing;
  for i in (1..29).rev() {
    let new_tick = ((tick_cumul_28x6h[i] - tick_cumul_28x6h[i-1]) / 21600) as i32;
    if new_tick < min_tick {
      min_tick = new_tick;
    } else if new_tick > max_tick {
      max_tick = new_tick;
    }
  }
  
  let mut range_top = f32::powf(1.0001, max_tick as f32);
  let mut range_bottom = f32::powf(1.0001, min_tick as f32);
  let amount0 = current_tick_liquidity as f32
    * (range_top.sqrt() - current_sqrt_price as f32)
    / (current_sqrt_price as f32 * range_top.sqrt())
    / f32::powf(10.0, t0_decimals as f32);
  let amount1 = current_tick_liquidity as f32
    * (current_sqrt_price as f32 - range_bottom.sqrt())
    / f32::powf(10.0, t1_decimals as f32);
  let mut est_tvl_in_range = amount0 * current_price + amount1;

  if token0 == "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2" {
    current_price = 1.0 / current_price;
    est_tvl_in_range = amount0 + amount1 * current_price;
    let new_range_top = 1.0 / range_bottom;
    let new_range_bottom = 1.0 / range_top;
    range_top = new_range_top;
    range_bottom = new_range_bottom;
  }
  
  let fees_24h_eth = fees_24h_usd / eth_price_usd;
  let edr = fees_24h_eth / est_tvl_in_range as f32;
  let edpr = edr * 100.0;

  let result = LensValue {
    address: address.to_string(),
    current_price,
    range_top,
    range_bottom,
    edpr
  };
  result
}
