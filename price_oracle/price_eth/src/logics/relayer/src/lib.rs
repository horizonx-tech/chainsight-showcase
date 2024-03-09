mod types;
mod canister;
mod config;

use ic_web3_rs::{self, types::U256};
pub type CallCanisterResponse = types::ResponseType;
pub type CallCanisterArgs = ();

#[derive(Default, Clone)]
struct Data {
    value: U256,
    timestamp: u64,
}

thread_local! {
    static LAST_RELAYED: std::cell::RefCell<Data> = std::cell::RefCell::new(Data::default());
}

const SCALE: u64 = 100;
const THRESHOLD: u64 = 5;
const INTERVAL_SECS: u64 = 3600;

pub fn call_args() -> CallCanisterArgs {
    ()
}

pub fn filter(res: &CallCanisterResponse) -> bool {
    let result = U256::from_dec_str(&res.value);
    if result.is_err() {
        return false;
    }

    let new = Data {
        value: result.unwrap(),
        timestamp: res.timestamp,
    };
    let scale = U256::from(SCALE);
    let threshold = U256::from(THRESHOLD);

    LAST_RELAYED.with_borrow_mut(|last| {
        if last.value.is_zero() {
            *last = new;
            return true;
        }
        if new.timestamp <= last.timestamp {
            return false;
        }
        let (ratio, _) = new.value.saturating_mul(scale).div_mod(last.value);
        if new.timestamp - last.timestamp > INTERVAL_SECS || ratio.abs_diff(scale).ge(&threshold) {
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
        let res = CallCanisterResponse {
            value: String::from("10000000000"),
            timestamp: 1704034800,
        };
        assert!(filter(&res));
        match U256::from_dec_str(&res.value) {
            Ok(value) => LAST_RELAYED.with_borrow(|last| {
                assert_eq!(last.value, value);
                assert_eq!(last.timestamp, res.timestamp);
            }),
            Err(err) => panic!("{}", err),
        }
    }

    #[test]
    fn test_update_threshold_positive() {
        let res = CallCanisterResponse {
            value: String::from("10000000000"),
            timestamp: 1704034800,
        };
        assert!(filter(&res));

        let res2 = CallCanisterResponse {
            value: String::from("10500000000"),
            timestamp: res.timestamp + 1,
        };
        assert!(filter(&res2));
        match U256::from_dec_str(&res2.value) {
            Ok(value) => LAST_RELAYED.with_borrow(|last| {
                assert_eq!(last.value, value);
                assert_eq!(last.timestamp, res2.timestamp);
            }),
            Err(err) => panic!("{}", err),
        }
    }

    #[test]
    fn test_update_threshold_negative() {
        let res = CallCanisterResponse {
            value: String::from("10000000000"),
            timestamp: 1704034800,
        };
        assert!(filter(&res));

        let res2 = CallCanisterResponse {
            value: String::from("9500000000"),
            timestamp: res.timestamp + 1,
        };
        assert!(filter(&res2));
        match U256::from_dec_str(&res2.value) {
            Ok(value) => LAST_RELAYED.with_borrow(|last| {
                assert_eq!(last.value, value);
                assert_eq!(last.timestamp, res2.timestamp);
            }),
            Err(err) => panic!("{}", err),
        }
    }

    #[test]
    fn test_update_interval() {
        let res = CallCanisterResponse {
            value: String::from("10000000000"),
            timestamp: 1704034800,
        };
        assert!(filter(&res));

        let res2 = CallCanisterResponse {
            value: String::from("10000000000"),
            timestamp: res.timestamp + 3601,
        };
        assert!(filter(&res2));
        match U256::from_dec_str(&res2.value) {
            Ok(value) => LAST_RELAYED.with_borrow(|last| {
                assert_eq!(last.value, value);
                assert_eq!(last.timestamp, res2.timestamp);
            }),
            Err(err) => panic!("{}", err),
        }
    }

    #[test]
    fn test_skip() {
        let res = CallCanisterResponse {
            value: String::from("10000000000"),
            timestamp: 1704034800,
        };
        assert!(filter(&res));

        let res2 = CallCanisterResponse {
            value: String::from("10499999999"),
            timestamp: res.timestamp + 3600,
        };
        assert!(!filter(&res2));

        let res3 = CallCanisterResponse {
            value: String::from("9600000000"),
            timestamp: res.timestamp + 3600,
        };
        assert!(!filter(&res3));
        match U256::from_dec_str(&res.value) {
            Ok(value) => LAST_RELAYED.with_borrow(|last| {
                assert_eq!(last.value.to_string(), value.to_string());
                assert_eq!(last.timestamp, res.timestamp);
            }),
            Err(err) => panic!("{}", err),
        }
    }

    #[test]
    fn test_ignore_old_timestamp() {
        let res = CallCanisterResponse {
            value: String::from("10000000000"),
            timestamp: 1704034800,
        };
        assert!(filter(&res));

        let res2 = CallCanisterResponse {
            value: String::from("10500000000"),
            timestamp: res.timestamp,
        };
        assert!(!filter(&res2));

        match U256::from_dec_str(&res.value) {
            Ok(value) => LAST_RELAYED.with_borrow(|last| {
                assert_eq!(last.value.to_string(), value.to_string());
                assert_eq!(last.timestamp, res.timestamp);
            }),
            Err(err) => panic!("{}", err),
        }
    }
}
