pub struct ParamVixPerTerm {
    pub variance: f64,
    pub t: f64,
    pub minites_until_t: f64,
}
pub struct ParamVix {
    pub near: ParamVixPerTerm,
    pub next: ParamVixPerTerm,
}

pub const N_30: f64 = 30.0 * 24.0 * 60.0; // 43200
pub const N_365: f64 = 365.0 * 24.0 * 60.0; // 525600

pub fn calculate_vix(p: ParamVix) -> f64 {
    let ParamVix { near, next } = p;
    let contribution_near = near.t * near.variance
        * ((next.minites_until_t - N_30) / (next.minites_until_t - near.minites_until_t));
    let contribution_next = next.t * next.variance
        * ((N_30 - near.minites_until_t) / (next.minites_until_t - near.minites_until_t));
    let inner_square_root = (contribution_near + contribution_next) * (N_365 / N_30);
    100.0 * inner_square_root.sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_vix_from_approximation() {
        // https://cdn.cboe.com/api/global/us_indices/governance/VIX_Methodology.pdf
        // -> Calculate the VIX Index
        assert_eq!(
            calculate_vix(ParamVix {
                near: ParamVixPerTerm {
                    variance: 0.019233906,
                    t: 0.0656088,
                    minites_until_t: 34484.0,
                },
                next: ParamVixPerTerm {
                    variance: 0.019423884,
                    t: 0.0855289,
                    minites_until_t: 44954.0,
                }
            }),
            13.927840480842871 // 13.93
        )
    }

    #[test]
    fn test_calculate_vix_from_measured() {
        assert_eq!(
            calculate_vix(ParamVix {
                near: ParamVixPerTerm {
                    variance: 0.019233906397055467,
                    t: 34484.0 / 525600.0,
                    minites_until_t: 34484.0,
                },
                next: ParamVixPerTerm {
                    variance: 0.01942388426632579,
                    t: 44954.0 / 525600.0,
                    minites_until_t: 44954.0,
                }
            }),
            13.927842342097524
        )
    }
}