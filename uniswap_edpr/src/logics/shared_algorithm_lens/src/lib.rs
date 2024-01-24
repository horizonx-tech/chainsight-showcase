use shared_algorithm_lens_accessors::*;

#[derive(Clone, Debug, Default, candid :: CandidType, serde :: Deserialize, serde :: Serialize)]
pub struct LensValue {
    pub address: String,    // The contract address for the trading pair
    pub current_price: f32, // The current price of the token of interest, expressed in ETH
    pub range_top: f32,     // Approximate highest price (in ETH) over the preceding week
    pub range_bottom: f32,  // Approximate lowest price (in ETH) over the preceding week
    pub edpr: f32,          // Estimated Daily Percentage Return
}

pub async fn calculate(targets: Vec<String>) -> LensValue {
    let pool_fees_result =
        get_get_last_snapshot_value_in_pool_fees(targets.get(0usize).unwrap().clone()).await;
    let tc28x6_result =
        get_get_last_snapshot_value_in_tcumul_28x6hr(targets.get(1usize).unwrap().clone()).await;
    let v3pool_result =
        get_get_last_snapshot_value_in_v3pool(targets.get(2usize).unwrap().clone()).await;
    let eth_usdc_price_result =
        get_get_last_snapshot_value_in_eth_usdc_price(targets.get(3usize).unwrap().clone()).await;

    let pool_fees_result_0 = &pool_fees_result.clone().unwrap().result[0];
    let fees_24h_usd = pool_fees_result_0.fees_24h_usd;
    let t0_decimals = pool_fees_result_0.pool_summary_level_1.t0_decimals;
    let t1_decimals = pool_fees_result_0.pool_summary_level_1.t1_decimals;

    let eth_price_usd = eth_usdc_price_result.clone().unwrap().result.t1_price_usd;

    let v3pool_result_res = v3pool_result.clone().unwrap().result;
    let address = v3pool_result_res.address;
    let current_tick_liquidity_str = v3pool_result_res.liquidity; // returns hex string - converted to integer in next line
    let current_tick_liquidity =
        i128::from_str_radix(current_tick_liquidity_str.trim_start_matches("0x"), 16).unwrap();
    let sqrt_ratio_x96_str = v3pool_result_res.sqrt_ratio_x96; // returns hex string - converted to integer in next line
    let sqrt_ratio_x96 =
        u128::from_str_radix(sqrt_ratio_x96_str.trim_start_matches("0x"), 16).unwrap();
    let tick_current = v3pool_result_res.tick_current;
    let token0 = v3pool_result_res.token0;

    let tick_cumul_28x6h = tc28x6_result.unwrap().0;

    let params = get_params(
        sqrt_ratio_x96,
        tick_current,
        current_tick_liquidity,
        token0,
        t0_decimals,
        t1_decimals,
        tick_cumul_28x6h,
    );
    let est_tvl_in_range = params.0;
    let current_price = params.1;
    let range_top = params.2;
    let range_bottom = params.3;

    if range_top == range_bottom {
        return LensValue {
            address: address.to_string(),
            current_price,
            range_top,
            range_bottom,
            edpr: 0.0,
        };
    }

    let fees_24h_eth = fees_24h_usd / eth_price_usd;
    let edr = fees_24h_eth / est_tvl_in_range as f32;
    let edpr = edr * 100.0;

    let result = LensValue {
        address: address.to_string(),
        current_price,
        range_top,
        range_bottom,
        edpr,
    };
    result
}

fn get_params(
    sqrt_ratio_x96: u128,
    tick_current: i32,
    current_tick_liquidity: i128,
    token0: String,
    t0_decimals: i64,
    t1_decimals: i64,
    tick_cumul_28x6h: Vec<i64>,
) -> (f32, f32, f32, f32) {
    let current_sqrt_price = sqrt_ratio_x96 as f32 / f32::powf(2.0, 96.0);
    let mut current_price = current_sqrt_price * current_sqrt_price;

    let mut min_tick = tick_current;
    let mut max_tick = tick_current;
    for i in (1..29).rev() {
        let new_tick = ((tick_cumul_28x6h[i] - tick_cumul_28x6h[i - 1]) / 21600) as i32;
        if new_tick < min_tick {
            min_tick = new_tick;
        } else if new_tick > max_tick {
            max_tick = new_tick;
        }
    }

    let mut range_top = f32::powf(1.0001, max_tick as f32);
    let mut range_bottom = f32::powf(1.0001, min_tick as f32);
    let amount0 = current_tick_liquidity as f32 * (range_top.sqrt() - current_sqrt_price as f32)
        / (current_sqrt_price as f32 * range_top.sqrt())
        / f32::powf(10.0, t0_decimals as f32);
    let amount1 = current_tick_liquidity as f32 * (current_sqrt_price as f32 - range_bottom.sqrt())
        / f32::powf(10.0, t1_decimals as f32);
    let mut est_tvl_in_range = amount0 * current_price + amount1;

    if token0.to_lowercase() == "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2" {
        let new_current_price = 1.0 / current_price;
        let new_est_tvl_in_range = amount0 + amount1 * new_current_price;
        let new_range_top = 1.0 / range_bottom;
        let new_range_bottom = 1.0 / range_top;
        current_price = new_current_price;
        est_tvl_in_range = new_est_tvl_in_range;
        range_top = new_range_top;
        range_bottom = new_range_bottom;
    }

    (est_tvl_in_range, current_price, range_top, range_bottom)
}

#[cfg(test)]
pub mod tests {

    use super::*;

    #[test]
    fn test_est_tvl_in_range() {
        // https://etherscan.io/address/0xc2e9f25be6257c210d7adf0d4cd6e3e881ba25f8#readContract#F8
        let tick_cumul_28x6h: Vec<i64> = vec![
            -6490142419541,
            -6491845467521,
            -6493539867881,
            -6495232991945,
            -6496926846521,
            -6498621135269,
            -6500317175945,
            -6502012511933,
            -6503706680753,
            -6505399692125,
            -6507090767129,
            -6508780873901,
            -6510472590869,
            -6512164927985,
            -6513857329301,
            -6515549074157,
            -6517241925473,
            -6518935812965,
            -6520632937889,
            -6522329705885,
            -6524024234585,
            -6525717868181,
            -6527410199417,
            -6529102265669,
            -6530794892681,
            -6532485781997,
            -6534171938249,
            -6535858342325,
            -6537545935601,
        ];

        // https://omni.oku.zone/ethereum?id=1&jsonrpc=2.0&method=cush_getV3Pool&params=["0xc2e9f25be6257c210d7adf0d4cd6e3e881ba25f8",19040779]
        let tick_current = -78176;
        let tick_spacing = 60;
        let sqrt_ratio_x96: u128 = 1590062295482777098773356604;
        let current_tick_liquidity: i128 = 1251982834387423730840398;
        let token0 = "0x6b175474e89094c44da98b954eedeac495271d0f";

        // https://omni.oku.zone/ethereum?id=1&jsonrpc=2.0&method=cush_getPoolFees&params=["0xc2e9f25be6257c210d7adf0d4cd6e3e881ba25f8"]
        let t0_decimals = 18;
        let t1_decimals = 18;

        // https://info.uniswap.org/#/pools/0xc2e9f25be6257c210d7adf0d4cd6e3e881ba25f8
        let eth_in_current_tick = 75.47;

        // calculation to be tested
        let params = get_params(
            sqrt_ratio_x96,
            tick_current,
            current_tick_liquidity,
            token0.to_string(),
            t0_decimals,
            t1_decimals,
            tick_cumul_28x6h,
        );
        let est_tvl_in_range = params.0;

        // comparator
        let min_tick = -78844;
        let max_tick = -78062;
        let expected_tvl_in_range =
            eth_in_current_tick * (max_tick as f32 - min_tick as f32) / tick_spacing as f32;

        // data sources are not identical, so 5% error is allowed in testing
        assert!(
            est_tvl_in_range > expected_tvl_in_range * 0.95
                && est_tvl_in_range < expected_tvl_in_range * 1.05
        );
    }
}
