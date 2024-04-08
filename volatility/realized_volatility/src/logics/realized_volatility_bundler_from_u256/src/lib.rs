use std::{collections::BTreeMap, ops::Mul};

use ic_web3_rs::futures;
use realized_volatility_bundler_from_u256_accessors::*;

pub type LensValue = BTreeMap<String, Result<f64, String>>;

#[derive(Clone, Debug, Default, candid :: CandidType, serde :: Deserialize, serde :: Serialize)]
pub struct CalculateArgs {
    // Specify by args to remove the number of targets
    pub targets: Vec<String>,
    pub snapshot_count: u64,
}
pub async fn calculate(_targets: Vec<String>, args: CalculateArgs) -> LensValue {
    let snapshots_vec = futures::future::join_all(
        args
            .targets
            .iter()
            .map(|dest| 
                get_snapshots(
                    dest.clone(),
                    args.snapshot_count,
                ),
            ),
    ).await;

    let res_vec = args.targets.iter().enumerate().map(|(i, target)| {
        let res_snapshots = snapshots_vec.get(i).unwrap();
        let result = match res_snapshots {
            Ok(snapshots) => {
                let prices = snapshots.iter().map(|v| v.parse::<u128>().unwrap()).collect::<Vec<u128>>();
                Ok(realized_volatility(prices))
            },
            Err(e) => Err(e.clone()),
        };
        (target.clone(), result)
    }).collect::<Vec<(String, Result<f64, String>)>>();
    res_vec.into_iter().collect()
}

pub fn realized_volatility(values: Vec<u128>) -> f64 {
    let size = values.len();
    let values = values.iter().map(|v| *v as f64).collect::<Vec<f64>>();

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
            623330981979783u128,
            600518686448770u128,
            607073626842525u128,
            637510465809231u128,
            638448574426221u128,
            639197943231740u128,
            638557370001757u128,
            640170391988596u128,
            652779104210042u128,
            696223851107765u128,
            694697603255450u128,
            650420714983778u128,
            623461872117769u128,
            595150509773174u128,
            586214431269636u128,
            604165004918448u128,
            596037019151431u128,
            557784962304499u128,
            567278343011584u128,
            560204368943828u128,
            575224857464874u128,
            553148684940906u128,
            575228525660004u128,
            550146883098981u128,
            570989152865942u128,
            573509572013513u128,
            563185894633847u128,
            582669634172006u128,
            563802345959968u128,
            557696621925851u128,
            557427617452429u128,
        ];

        let val = realized_volatility(prices.clone());
        assert!(val >= 18.09f64 && val <= 18.1f64);
    }

    #[test]
    fn test_calculate_wethusdc_20230514_hourly() {
        let prices = vec![
            1801349421u128,
            1794848728u128,
            1799887075u128,
            1804137694u128,
            1803190133u128,
            1804140366u128,
            1807591144u128,
            1806638192u128,
            1803747976u128,
            1804294768u128,
            1803149078u128,
            1803026184u128,
            1810755481u128,
            1811181380u128,
            1809058903u128,
            1820134411u128,
            1808705134u128,
            1806894463u128,
            1801895027u128,
            1796657140u128,
            1797299200u128,
            1798347300u128,
            1801126350u128,
        ];
        let val = realized_volatility(prices.clone());
        assert!(val >= 1.227f64 && val <= 1.228f64);
    }
}