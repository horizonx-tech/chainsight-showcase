use std::cell::RefCell;
// 0.5%
const PRICE_CHANGE_THRESHOLD: u128 = 5;

use crate::CallCanisterResponse;
pub fn filter(res: &CallCanisterResponse) -> bool {
    let last = last_price();
    set_last_price(*res);
    let diff = if last > *res {
        last - *res
    } else {
        *res - last
    };
    let threshold = (last * PRICE_CHANGE_THRESHOLD) / 1000;
    diff >= threshold
}
thread_local! {
    static LAST_PRICE: RefCell<u128> = RefCell::new(0);
}

fn last_price() -> u128 {
    LAST_PRICE.with(|v| *v.borrow())
}

fn set_last_price(v: u128) {
    LAST_PRICE.with(|val| *val.borrow_mut() = v);
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn test_filter_true() {
        let res = 1005;
        let last = 1000;
        set_last_price(last);
        let f = filter(&res);
        assert_eq!(f, true);
    }
    #[test]
    fn test_filter_false() {
        let res = 1005;
        let last = 1001;
        set_last_price(last);
        let f = filter(&res);
        assert_eq!(f, false);
    }
}
