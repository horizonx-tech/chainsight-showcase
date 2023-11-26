use index_lens_accessors::*;
use index_lens_bindings::market_indexer_coinmarketcap::{CurrencyData, Snapshot};
const DIVISOR: f64 = 10000000.0;
const DECIMALS: i32 = 6;
#[derive(Clone, Debug, Default, candid :: CandidType, serde :: Deserialize, serde :: Serialize)]
pub struct LensValue {
    pub index: f64,
}
pub async fn calculate(targets: Vec<String>) -> LensValue {
    let result =
        get_get_last_snapshot_in_market_indexer_coinmarketcap(targets.get(0usize).unwrap().clone())
            .await
            .unwrap();
    LensValue {
        index: calculate_index_value(result),
    }
}

fn calculate_index_value(snapshot: Box<Snapshot>) -> f64 {
    let values = vec![
        snapshot.value.data._1,
        snapshot.value.data._2,
        snapshot.value.data._3,
        snapshot.value.data._4,
        snapshot.value.data._5,
        snapshot.value.data._6,
        snapshot.value.data._7,
        snapshot.value.data._8,
        snapshot.value.data._9,
        snapshot.value.data._10,
        snapshot.value.data._11,
        snapshot.value.data._12,
        snapshot.value.data._13,
        snapshot.value.data._14,
        snapshot.value.data._15,
        snapshot.value.data._16,
        snapshot.value.data._17,
        snapshot.value.data._18,
        snapshot.value.data._19,
        snapshot.value.data._20,
    ];
    let (market_caps_with_iwf, total_market_cap_with_iwf) = calc_market_caps_with_iwf(values);
    let market_caps_with_iwf_weighted = market_caps_with_iwf
        .into_iter()
        .map(|s| s * s / total_market_cap_with_iwf);

    let v = market_caps_with_iwf_weighted.clone().sum::<f64>()
        / (market_caps_with_iwf_weighted.len() as f64)
        / DIVISOR;

    (v * 10_f64.powi(DECIMALS)).round() / 10_f64.powi(DECIMALS)
}

fn calc_market_caps_with_iwf(values: Vec<CurrencyData>) -> (Vec<f64>, f64) {
    let mut total_market_cap_with_iwf: f64 = 0.0;
    let market_caps_with_iwf = values.iter().map(|s| {
        let market_cap = s.quote.usd.market_cap as f64;
        let circulating_supply = s.circulating_supply;
        let total_supply = s.total_supply;
        let iwf = circulating_supply / total_supply;
        let market_cap_with_iwf = market_cap as f64 * iwf;
        total_market_cap_with_iwf += market_cap_with_iwf;
        market_cap_with_iwf
    });
    (market_caps_with_iwf.collect(), total_market_cap_with_iwf)
}

#[cfg(test)]
mod tests {

    use index_lens_bindings::market_indexer_coinmarketcap::{
        CurrencyData, Inner, Quote, QuoteData, Snapshot, SnapshotValue,
    };

    use crate::calculate_index_value;

    #[test]
    fn test_calculate_index_value() {
        let snapshot = Snapshot {
            timestamp: 0,
            value: Box::new(SnapshotValue {
                data: Inner {
                    _1: CurrencyData {
                        circulating_supply: 195223531.0,
                        total_supply: 21200000.2,
                        quote: Box::new(Quote {
                            usd: Box::new(QuoteData {
                                market_cap: 46407156443.0,
                            }),
                        }),
                    },
                    _2: CurrencyData {
                        circulating_supply: 18729881.0,
                        total_supply: 21100000.0,
                        quote: Box::new(Quote {
                            usd: Box::new(QuoteData {
                                market_cap: 46407156443.0,
                            }),
                        }),
                    },
                    _3: CurrencyData {
                        circulating_supply: 18729881.0,
                        total_supply: 21000000.0,
                        quote: Box::new(Quote {
                            usd: Box::new(QuoteData {
                                market_cap: 46407156443.0,
                            }),
                        }),
                    },
                    _4: CurrencyData {
                        circulating_supply: 18729881.0,
                        total_supply: 21000000.0,
                        quote: Box::new(Quote {
                            usd: Box::new(QuoteData {
                                market_cap: 46407156443.0,
                            }),
                        }),
                    },
                    _5: CurrencyData {
                        circulating_supply: 18729881.0,
                        total_supply: 21000000.0,
                        quote: Box::new(Quote {
                            usd: Box::new(QuoteData {
                                market_cap: 46407156443.0,
                            }),
                        }),
                    },
                    _6: CurrencyData {
                        circulating_supply: 18729881.0,
                        total_supply: 21000000.0,
                        quote: Box::new(Quote {
                            usd: Box::new(QuoteData {
                                market_cap: 46407156443.0,
                            }),
                        }),
                    },
                    _7: CurrencyData {
                        circulating_supply: 18729881.0,
                        total_supply: 21000000.0,
                        quote: Box::new(Quote {
                            usd: Box::new(QuoteData {
                                market_cap: 46407156443.0,
                            }),
                        }),
                    },
                    _8: CurrencyData {
                        circulating_supply: 18729881.0,
                        total_supply: 21000000.0,
                        quote: Box::new(Quote {
                            usd: Box::new(QuoteData {
                                market_cap: 46407156443.0,
                            }),
                        }),
                    },
                    _9: CurrencyData {
                        circulating_supply: 18729881.0,
                        total_supply: 21000000.0,
                        quote: Box::new(Quote {
                            usd: Box::new(QuoteData {
                                market_cap: 46407156443.0,
                            }),
                        }),
                    },
                    _10: CurrencyData {
                        circulating_supply: 18729881.0,
                        total_supply: 21000000.0,
                        quote: Box::new(Quote {
                            usd: Box::new(QuoteData {
                                market_cap: 46407156443.0,
                            }),
                        }),
                    },
                    _11: CurrencyData {
                        circulating_supply: 18729881.0,
                        total_supply: 21000000.0,
                        quote: Box::new(Quote {
                            usd: Box::new(QuoteData { market_cap: 0.0 }),
                        }),
                    },
                    _12: CurrencyData {
                        circulating_supply: 18729881.0,
                        total_supply: 21000000.0,
                        quote: Box::new(Quote {
                            usd: Box::new(QuoteData { market_cap: 0.0 }),
                        }),
                    },
                    _13: CurrencyData {
                        circulating_supply: 18729881.0,
                        total_supply: 21000000.0,
                        quote: Box::new(Quote {
                            usd: Box::new(QuoteData { market_cap: 0.0 }),
                        }),
                    },
                    _14: CurrencyData {
                        circulating_supply: 18729881.0,
                        total_supply: 21000000.0,
                        quote: Box::new(Quote {
                            usd: Box::new(QuoteData { market_cap: 0.0 }),
                        }),
                    },
                    _15: CurrencyData {
                        circulating_supply: 18729881.0,
                        total_supply: 21000000.0,
                        quote: Box::new(Quote {
                            usd: Box::new(QuoteData { market_cap: 0.0 }),
                        }),
                    },
                    _16: CurrencyData {
                        circulating_supply: 18729881.0,
                        total_supply: 21000000.0,
                        quote: Box::new(Quote {
                            usd: Box::new(QuoteData { market_cap: 0.0 }),
                        }),
                    },
                    _17: CurrencyData {
                        circulating_supply: 18729881.0,
                        total_supply: 21000000.0,
                        quote: Box::new(Quote {
                            usd: Box::new(QuoteData { market_cap: 0.0 }),
                        }),
                    },
                    _18: CurrencyData {
                        circulating_supply: 18729881.0,
                        total_supply: 21000000.0,
                        quote: Box::new(Quote {
                            usd: Box::new(QuoteData { market_cap: 0.0 }),
                        }),
                    },
                    _19: CurrencyData {
                        circulating_supply: 18729881.0,
                        total_supply: 21000000.0,
                        quote: Box::new(Quote {
                            usd: Box::new(QuoteData { market_cap: 0.0 }),
                        }),
                    },
                    _20: CurrencyData {
                        circulating_supply: 18729881.0,
                        total_supply: 21000000.0,
                        quote: Box::new(Quote {
                            usd: Box::new(QuoteData { market_cap: 0.0 }),
                        }),
                    },
                },
            }),
        };
        let result = calculate_index_value(Box::new(snapshot));
        assert_eq!(result, 1238.194068);
    }
}
