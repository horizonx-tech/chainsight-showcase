#[derive(Clone, Debug, PartialEq)]
pub struct Option {
    pub strike_price: f64,
    pub bid: f64,
    pub ask: f64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ParamVariance {
    pub options: Vec<Option>,

    pub time_to_expiration: f64, // T
    pub risk_free_rate: f64, // R
    pub forward_price: f64,
    pub k_0: f64,
}
pub fn variance_per_term(p: ParamVariance) -> f64 {
    let mut options_for_contribution = vec![];
    let last_idx = p.options.len() - 1;
    for (idx, option) in p.options.iter().enumerate() {
        let delta_k = match idx {
            idx if idx == last_idx => {
                option.strike_price - p.options[idx - 1].strike_price
            }
            idx if idx == 0 => {
                p.options[idx + 1].strike_price - option.strike_price
            },
            _ => {
                (p.options[idx + 1].strike_price - p.options[idx - 1].strike_price) / 2.0
            }
        };
        let mid_price = (option.bid + option.ask) / 2.0;

        options_for_contribution.push(OptionForContribution {
            strike_price: option.strike_price,
            mid_price,
            delta_k,
        });
    }

    let left = variance_left_part(options_for_contribution, p.risk_free_rate, p.time_to_expiration);
    let right = variance_right_part(p.forward_price, p.k_0, p.time_to_expiration);
    left - right
}
fn variance_left_part(options: Vec<OptionForContribution>, risk_free_rate: f64, time_to_expiration: f64) -> f64 {
    let sum_contributions = calculate_sum_contributions(options, risk_free_rate, time_to_expiration);
    sum_contributions * 2.0 / time_to_expiration
}
fn variance_right_part(f: f64, k_0: f64, t: f64) -> f64 {
    (f / k_0 - 1.0).powi(2) / t
}

#[derive(Clone, Debug, PartialEq)]
pub struct OptionForContribution {
    pub strike_price: f64,
    pub mid_price: f64,
    pub delta_k: f64,
}
fn calculate_sum_contributions(options: Vec<OptionForContribution>, risk_free_rate: f64, time_to_expiration: f64) -> f64 {
    let mut sum_contributions = 0.0;
    for option in options {
        sum_contributions += contribution_per_option(option, risk_free_rate, time_to_expiration);
    }
    sum_contributions
}
fn contribution_per_option(option: OptionForContribution, risk_free_rate: f64, time_to_expiration: f64) -> f64 {
    let numerator = option.delta_k * (risk_free_rate * time_to_expiration).exp() * option.mid_price;
    numerator / option.strike_price.powf(2.0)
}

#[cfg(test)]
mod tests {
    use crate::calc::k::{calculate_f, ParamF, find_closest_less_than_f};

    use super::*;

    // https://cdn.cboe.com/api/global/us_indices/governance/VIX_Methodology.pdf

    #[test]
    fn test_contribution_per_option() {
        // 1370 Put
        let mid_price = (0.05 + 0.35) / 2.0; // (call + put) / 2
        let delta_k = 1375.0 - 1370.0; // NOTE: use ki because this option is the lowest
        assert_eq!(
            contribution_per_option(
                OptionForContribution {
                    strike_price: 1370.0,
                    mid_price,
                    delta_k
                },
                0.00031664, // risk_free_rate
                34484.0 / 525600.0, // time_to_expiration
            ),
            0.0000005328045045527672 // 0.0000005328
        )
    }

    #[test]
    fn test_variance_right_part_near_term() {
        let t1 = 34484.0 / 525600.0;
        let r1 = 0.00031664;
        let f = calculate_f(ParamF {
            strike_price: 1965.0,
            call_price: 21.05,
            put_price: 23.15,
            risk_free_rate: r1,
            time_to_expiration: t1,
        });
        let list = vec![
            1955.0,
            1960.0,
            1965.0,
            1970.0
        ];
        let k_0 = list.get(find_closest_less_than_f(f, list.clone()).unwrap()).unwrap();

        assert_eq!(
            variance_right_part(f, *k_0, t1),
            0.0000333663350403073 // 0.00003337
        );
    }

    #[test]
    fn test_variance_right_part_next_term() {
        let t2 = 44954.0 / 525600.0;
        let r2 = 0.00028797;
        let f = calculate_f(ParamF {
            strike_price: 1960.0,
            call_price: 27.30,
            put_price: 24.90,
            risk_free_rate: r2,
            time_to_expiration: t2,
        });
        let list = vec![
            1955.0,
            1960.0,
            1965.0,
            1970.0
        ];
        let k_0 = list.get(find_closest_less_than_f(f, list.clone()).unwrap()).unwrap();
        
        assert_eq!(
            variance_right_part(f, *k_0, t2),
            0.000017531486804492088 // 0.00001753
        );
    }
}
