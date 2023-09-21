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
    pub data: Data,
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
pub struct Data {
    body: Body,
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
pub struct Body {
    data: BodyData,
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
pub struct BodyData {
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
    use crate::app::{Body, BodyData};

    use super::{CurrencyData, Data, Quote, QuoteData, SnapshotValue};
    use serde_json;
    #[test]
    fn test_deser_json() {
        let input = r#"{
            "data": {
                "body": {
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
            }
        }
        "#;
        let want = SnapshotValue {
            data: Data {
                body: Body {
                    data: BodyData {
                        data: CurrencyData {
                            quote: Quote {
                                usd: QuoteData {
                                    price: 0.057078441426201255,
                                    volume_24h: 14019308.19738592,
                                },
                            },
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
            "code": "000000",
            "message": null,
            "messageDetail": null,
            "data": {
              "body": {
                "data": {
                  "22265": {
                    "symbol": "USDT",
                    "circulating_supply": 870344347,
                    "last_updated": "2023-08-11T09:08:00.000Z",
                    "is_active": 1,
                    "total_supply": 10000000000,
                    "cmc_rank": 371,
                    "tags": [],
                    "date_added": "2022-10-18T06:35:35.000Z",
                    "quote": {
                      "USD": {
                        "fully_diluted_market_cap": 550783080.47,
                        "last_updated": "2023-08-11T09:08:00.000Z",
                        "market_cap_dominance": 0,
                        "percent_change_30d": -13.28844569,
                        "percent_change_1h": -3.90850821,
                        "percent_change_24h": 7.79512402,
                        "market_cap": 47937094.05140087,
                        "volume_change_24h": 2360.9168,
                        "price": 0.055078308047424904,
                        "percent_change_60d": -14.55002285,
                        "volume_24h": 15409586.15963326,
                        "percent_change_90d": -31.36108765,
                        "percent_change_7d": 9.68612452
                      }
                    },
                    "num_market_pairs": 19,
                    "infinite_supply": false,
                    "name": "Tether USD",
                    "max_supply": 10000000000,
                    "id": 22265,
                    "is_fiat": 0,
                    "slug": "tether"
                  }
                },
                "status": {
                  "elapsed": 46,
                  "credit_count": 1,
                  "error_code": 0,
                  "timestamp": "2023-08-11T09:10:09.189Z",
                  "notice": "You have used 157% of your plan's monthly credit limit."
                }
              },
              "success": true
            },
            "success": true
          }
        "#;
        let want = SnapshotValue {
            data: Data {
                body: Body {
                    data: BodyData {
                        data: CurrencyData {
                            quote: Quote {
                                usd: QuoteData {
                                    price: 0.055078308047424904,
                                    volume_24h: 15409586.15963326,
                                },
                            },
                        },
                    },
                },
            },
        };
        let got: SnapshotValue = serde_json::from_str(input).unwrap();
        assert_eq!(want, got);
    }
}
