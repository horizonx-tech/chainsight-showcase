use candid::{Decode, Encode};
use chainsight_cdk_macros::StableMemoryStorable;
#[derive(
    Debug,
    Clone,
    candid::CandidType,
    candid::Deserialize,
    serde::Serialize,
    StableMemoryStorable,
    PartialEq,
)]
pub struct SnapshotValue {
    pub data: Inner,
}

#[derive(
    Debug,
    Clone,
    candid::CandidType,
    candid::Deserialize,
    serde::Serialize,
    StableMemoryStorable,
    PartialEq,
)]
pub struct Inner {
    #[serde(alias = "22265", alias = "data")]
    data: CurrencyData,
}

#[derive(
    Debug,
    Clone,
    candid::CandidType,
    candid::Deserialize,
    serde::Serialize,
    StableMemoryStorable,
    PartialEq,
)]
pub struct CurrencyData {
    quote: Quote,
}

#[derive(
    Debug,
    Clone,
    candid::CandidType,
    candid::Deserialize,
    serde::Serialize,
    StableMemoryStorable,
    PartialEq,
)]
pub struct Quote {
    #[serde(alias = "USD", alias = "usd")]
    usd: QuoteData,
}

#[derive(
    Debug,
    Clone,
    candid::CandidType,
    candid::Deserialize,
    serde::Serialize,
    StableMemoryStorable,
    PartialEq,
)]
pub struct QuoteData {
    price: f64,
    volume_24h: f64,
}

#[cfg(test)]
pub mod tests {
    use super::{CurrencyData, Inner, Quote, QuoteData, SnapshotValue};
    use serde_json;
    #[test]
    fn test_deser_json() {
        let input = r#"{
            "data": {
                "data": {
                    "quote": {
                        "USD": {
                            "price": 0.057078441426201255,
                            "volume_24h": 14019308.19738592
                        }
                    }
                }
            }
        }
        "#;
        let want = SnapshotValue {
            data: Inner {
                data: CurrencyData {
                    quote: Quote {
                        usd: QuoteData {
                            price: 0.057078441426201255,
                            volume_24h: 14019308.19738592,
                        },
                    },
                },
            },
        };
        let got: SnapshotValue = serde_json::from_str(input).unwrap();
        assert_eq!(want, got);
    }

    #[test]
    fn test_deser_json2() {
        let input = r#"{
            
            "data": {
                "22265": {
                    "id": 22265,
                    "name": "Tether USD",
                    "symbol": "USDT",
                    "slug": "tether",
                    "num_market_pairs": 19,
                    "date_added": "2022-10-18T06:35:35.000Z",
                    "tags": [],
                    "max_supply": 10000000000,
                    "circulating_supply": 870344347,
                    "total_supply": 10000000000,
                    "is_active": 1,
                    "infinite_supply": false,
                    "platform": null,
                    "cmc_rank": 366,
                    "is_fiat": 0,
                    "self_reported_circulating_supply": null,
                    "self_reported_market_cap": null,
                    "tvl_ratio": null,
                    "last_updated": "2023-08-11T07:12:00.000Z",
                    "quote": {
                        "USD": {
                            "price": 0.057078441426201255,
                            "volume_24h": 14019308.19738592,
                            "volume_change_24h": 2043.5138,
                            "percent_change_1h": 0.04879776,
                            "percent_change_24h": 11.49378424,
                            "percent_change_7d": 13.36133965,
                            "percent_change_30d": -10.31773588,
                            "percent_change_60d": -12.33273474,
                            "percent_change_90d": -29.71504806,
                            "market_cap": 49677898.830864884,
                            "market_cap_dominance": 0,
                            "fully_diluted_market_cap": 570784414.26,
                            "tvl": null,
                            "last_updated": "2023-08-11T07:12:00.000Z"
                        }
                    }
                }
            }
        }
        "#;
        let want = SnapshotValue {
            data: Inner {
                data: CurrencyData {
                    quote: Quote {
                        usd: QuoteData {
                            price: 0.057078441426201255,
                            volume_24h: 14019308.19738592,
                        },
                    },
                },
            },
        };
        let got: SnapshotValue = serde_json::from_str(input).unwrap();
        assert_eq!(want, got);
    }
}
