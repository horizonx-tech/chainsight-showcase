use std::ops::Mul;

use realized_volatility_for_chainsight20_accessors::*;

pub type LensValue = f64;
#[derive(Clone, Debug, Default, candid :: CandidType, serde :: Deserialize, serde :: Serialize)]
pub struct CalculateArgs {
    pub snapshot_count: u64
}
pub async fn calculate(targets: Vec<String>, args: CalculateArgs) -> LensValue {
    let result = get_snapshots_chainsight20(
        targets.get(0usize).unwrap().clone(),
        args.snapshot_count,
    )
    .await.expect("get_snapshots failed");
    
    realized_volatility(
        result.iter().map(|v| v.index).collect(),
        // NOTE: Specific Considerations for Chainsight20 Snapshots
    )
}

pub fn realized_volatility(values: Vec<f64>) -> f64 {
    let size = values.len();

    let mut all_squared_r: Vec<f64> = Vec::with_capacity(size - 1);
    for i in 1..size {
        let pt = values[i] as f64;
        let pt_minus_1 = values[i - 1] as f64;
        let r = (pt / pt_minus_1).ln().mul(100_f64);
        all_squared_r.push(r.powi(2));
    }

    let sum_of_squared_r = all_squared_r.iter().sum::<f64>();
    (sum_of_squared_r / size as f64).sqrt() * (size as f64).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_usdceth_202303_daily() {
        let prices = vec![
            623330981979783.0 as f64,
            600518686448770.0 as f64,
            607073626842525.0 as f64,
            637510465809231.0 as f64,
            638448574426221.0 as f64,
            639197943231740.0 as f64,
            638557370001757.0 as f64,
            640170391988596.0 as f64,
            652779104210042.0 as f64,
            696223851107765.0 as f64,
            694697603255450.0 as f64,
            650420714983778.0 as f64,
            623461872117769.0 as f64,
            595150509773174.0 as f64,
            586214431269636.0 as f64,
            604165004918448.0 as f64,
            596037019151431.0 as f64,
            557784962304499.0 as f64,
            567278343011584.0 as f64,
            560204368943828.0 as f64,
            575224857464874.0 as f64,
            553148684940906.0 as f64,
            575228525660004.0 as f64,
            550146883098981.0 as f64,
            570989152865942.0 as f64,
            573509572013513.0 as f64,
            563185894633847.0 as f64,
            582669634172006.0 as f64,
            563802345959968.0 as f64,
            557696621925851.0 as f64,
            557427617452429.0 as f64,
        ];

        let val = realized_volatility(prices.clone());
        assert!(val >= 18.09f64 && val <= 18.1f64);
    }

    #[test]
    fn test_calculate_wethusdc_20230514_hourly() {
        let prices = vec![
            1801349421.0 as f64,
            1794848728.0 as f64,
            1799887075.0 as f64,
            1804137694.0 as f64,
            1803190133.0 as f64,
            1804140366.0 as f64,
            1807591144.0 as f64,
            1806638192.0 as f64,
            1803747976.0 as f64,
            1804294768.0 as f64,
            1803149078.0 as f64,
            1803026184.0 as f64,
            1810755481.0 as f64,
            1811181380.0 as f64,
            1809058903.0 as f64,
            1820134411.0 as f64,
            1808705134.0 as f64,
            1806894463.0 as f64,
            1801895027.0 as f64,
            1796657140.0 as f64,
            1797299200.0 as f64,
            1798347300.0 as f64,
            1801126350.0 as f64,
        ];
        let val = realized_volatility(prices.clone());
        assert!(val >= 1.227f64 && val <= 1.228f64);
    }
}