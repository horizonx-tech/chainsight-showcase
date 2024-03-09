use ic_web3_rs::types::U256;
use std::{cell::RefCell, collections::HashMap};

#[derive(Clone, Debug, Default, candid :: CandidType, serde :: Deserialize, serde :: Serialize)]
pub struct LensValue {
    pub value: Option<chainsight_cdk::core::U256>,
}

pub struct Data {
    pub value: U256,
    pub timestamp: u64,
}

#[derive(Clone, Debug, Default, candid :: CandidType, serde :: Deserialize, serde :: Serialize)]
pub struct CalculateArgs {
    pub value: chainsight_cdk::core::U256,
    pub threshold: u32,
    pub scale: u32,
    pub interval_secs: u64,
    pub key: [u8; 32usize],
}

thread_local! {
    static LAST_RELAYED: RefCell<HashMap<[u8;32usize], Data>> = RefCell::new(HashMap::new());
}

pub async fn calculate(_targets: Vec<String>, args: CalculateArgs) -> LensValue {
    if filter(&args, ic_cdk::api::time()) {
        return LensValue {
            value: Some(args.value),
        };
    }

    return LensValue { value: None };
}

fn filter(args: &CalculateArgs, timestamp: u64) -> bool {
    let new = Data {
        value: U256::from_dec_str(&args.value.value().to_string()).unwrap(),
        timestamp,
    };
    let scale = U256::from(args.scale);
    let threshold = U256::from(args.threshold);

    LAST_RELAYED.with_borrow_mut(|map| {
        let e = map.get_mut(&args.key);
        if e.is_none() {
            map.insert(args.key, new);
            return true;
        }
        let last = e.unwrap();
        let (ratio, _) = new.value.saturating_mul(scale).div_mod(last.value);
        if new.timestamp - last.timestamp >= args.interval_secs
            || ratio.abs_diff(scale).ge(&threshold)
        {
            *last = new;
            true
        } else {
            false
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize() {
        let value = U256::from_dec_str("10000000000").unwrap();
        let timestamp = 1704034800;
        let args = CalculateArgs {
            value: chainsight_cdk::core::U256::from(value),
            threshold: 5,
            scale: 1000,
            interval_secs: 3600,
            key: [0u8; 32usize],
        };
        assert!(filter(&args, timestamp));
        LAST_RELAYED.with_borrow(|map| {
            let last = map.get(&args.key).unwrap();
            assert_eq!(last.value, value);
            assert_eq!(last.timestamp, timestamp);
        })
    }

    #[test]
    fn test_update_threshold_positive() {
        let value = U256::from_dec_str("10000000000").unwrap();
        let timestamp = 1704034800;
        let args = CalculateArgs {
            value: chainsight_cdk::core::U256::from(value),
            threshold: 5,
            scale: 1000,
            interval_secs: 3600,
            key: [0u8; 32usize],
        };
        assert!(filter(&args, timestamp));

        let value2 = U256::from_dec_str("10050000000").unwrap();
        let mut args2 = args.clone();
        args2.value = chainsight_cdk::core::U256::from(value2);
        let timestamp2 = timestamp + args.interval_secs;
        assert!(filter(&args2, timestamp2));
        LAST_RELAYED.with_borrow(|map| {
            let last = map.get(&args.key).unwrap();
            assert_eq!(last.value, value2);
            assert_eq!(last.timestamp, timestamp2);
        })
    }

    #[test]
    fn test_update_threshold_negative() {
        let value = U256::from_dec_str("10000000000").unwrap();
        let timestamp = 1704034800;
        let args = CalculateArgs {
            value: chainsight_cdk::core::U256::from(value),
            threshold: 5,
            scale: 1000,
            interval_secs: 3600,
            key: [0u8; 32usize],
        };
        assert!(filter(&args, timestamp));

        let value2 = U256::from_dec_str("9950000000").unwrap();
        let mut args2 = args.clone();
        args2.value = chainsight_cdk::core::U256::from(value2);
        let timestamp2 = timestamp + args.interval_secs;
        assert!(filter(&args2, timestamp2));
        LAST_RELAYED.with_borrow(|map| {
            let last = map.get(&args.key).unwrap();
            assert_eq!(last.value, value2);
            assert_eq!(last.timestamp, timestamp2);
        })
    }

    #[test]
    fn test_update_interval() {
        let value = U256::from_dec_str("10000000000").unwrap();
        let timestamp = 1704034800;
        let args = CalculateArgs {
            value: chainsight_cdk::core::U256::from(value),
            threshold: 5,
            scale: 1000,
            interval_secs: 3600,
            key: [0u8; 32usize],
        };
        assert!(filter(&args, timestamp));

        let value2 = U256::from_dec_str("10000000000").unwrap();
        let mut args2 = args.clone();
        args2.value = chainsight_cdk::core::U256::from(value2);
        let timestamp2 = timestamp + args.interval_secs;
        assert!(filter(&args2, timestamp2));
        LAST_RELAYED.with_borrow(|map| {
            let last = map.get(&args.key).unwrap();
            assert_eq!(last.value, value2);
            assert_eq!(last.timestamp, timestamp2);
        })
    }

    #[test]
    fn test_skip() {
        let value = U256::from_dec_str("10000000000").unwrap();
        let timestamp = 1704034800;
        let args = CalculateArgs {
            value: chainsight_cdk::core::U256::from(value),
            threshold: 5,
            scale: 1000,
            interval_secs: 3600,
            key: [0u8; 32usize],
        };
        assert!(filter(&args, timestamp));

        let mut args2 = args.clone();
        args2.value = chainsight_cdk::core::U256::from(U256::from_dec_str("10049999999").unwrap());
        let timestamp2 = timestamp + args.interval_secs - 1;
        
        assert!(!filter(&args2, timestamp2));
        LAST_RELAYED.with_borrow(|map| {
            let last = map.get(&args.key).unwrap();
            assert_eq!(last.value, value);
            assert_eq!(last.timestamp, timestamp);
        });

        let mut args3 = args.clone();
        args3.value = chainsight_cdk::core::U256::from(U256::from_dec_str("9960000000").unwrap());
        let timestamp3 = timestamp + args.interval_secs - 1;        
        assert!(!filter(&args3, timestamp3));
        LAST_RELAYED.with_borrow(|map| {
            let last = map.get(&args.key).unwrap();
            assert_eq!(last.value, value);
            assert_eq!(last.timestamp, timestamp);
        })
    }

    #[test]
    fn test_ignore_old_timestamp() {
        let value = U256::from_dec_str("10000000000").unwrap();
        let timestamp = 1704034800;
        let args = CalculateArgs {
            value: chainsight_cdk::core::U256::from(value),
            threshold: 5,
            scale: 1000,
            interval_secs: 3600,
            key: [0u8; 32usize],
        };
        assert!(filter(&args, timestamp));

        let mut args2 = args.clone();
        args2.value = chainsight_cdk::core::U256::from(U256::from_dec_str("10049999999").unwrap());
        
        assert!(!filter(&args2, timestamp));
        LAST_RELAYED.with_borrow(|map| {
            let last = map.get(&args.key).unwrap();
            assert_eq!(last.value, value);
            assert_eq!(last.timestamp, timestamp);
        });

        let mut args3 = args.clone();
        args3.value = chainsight_cdk::core::U256::from(U256::from_dec_str("9960000000").unwrap());        
        assert!(!filter(&args3, timestamp));
        LAST_RELAYED.with_borrow(|map| {
            let last = map.get(&args.key).unwrap();
            assert_eq!(last.value, value);
            assert_eq!(last.timestamp, timestamp);
        })
    }
}
