#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod types {
    use relayer_bindings as bindings;
    pub type ResponseType = bindings::ResponseType;
}
use ic_web3_rs::{self, types::U256};
pub type CallCanisterResponse = types::ResponseType;
pub type CallCanisterArgs = ();
struct Data {
    value: U256,
    timestamp: u64,
}
#[automatically_derived]
impl ::core::default::Default for Data {
    #[inline]
    fn default() -> Data {
        Data {
            value: ::core::default::Default::default(),
            timestamp: ::core::default::Default::default(),
        }
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Data {
    #[inline]
    fn clone(&self) -> Data {
        Data {
            value: ::core::clone::Clone::clone(&self.value),
            timestamp: ::core::clone::Clone::clone(&self.timestamp),
        }
    }
}
const LAST_RELAYED: ::std::thread::LocalKey<std::cell::RefCell<Data>> = {
    #[inline]
    fn __init() -> std::cell::RefCell<Data> {
        std::cell::RefCell::new(Data::default())
    }
    #[inline]
    unsafe fn __getit(
        init: ::std::option::Option<&mut ::std::option::Option<std::cell::RefCell<Data>>>,
    ) -> ::std::option::Option<&'static std::cell::RefCell<Data>> {
        #[thread_local]
        static __KEY: ::std::thread::local_impl::Key<std::cell::RefCell<Data>> = ::std::thread::local_impl::Key::<
            std::cell::RefCell<Data>,
        >::new();
        unsafe {
            __KEY
                .get(move || {
                    if let ::std::option::Option::Some(init) = init {
                        if let ::std::option::Option::Some(value) = init.take() {
                            return value;
                        } else if true {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("missing default value")
                                    ),
                                );
                            };
                        }
                    }
                    __init()
                })
        }
    }
    unsafe { ::std::thread::LocalKey::new(__getit) }
};
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
    LAST_RELAYED
        .with_borrow_mut(|last| {
            if last.value.is_zero() {
                *last = new;
                return true;
            }
            if new.timestamp <= last.timestamp {
                return false;
            }
            let (ratio, _) = new.value.saturating_mul(scale).div_mod(last.value);
            if new.timestamp - last.timestamp > INTERVAL_SECS
                || ratio.abs_diff(scale).ge(&threshold)
            {
                *last = new;
                true
            } else {
                false
            }
        })
}
