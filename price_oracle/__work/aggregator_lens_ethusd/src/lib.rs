use candid::CandidType;
use chainsight_cdk::lens::LensFinder;
use chainsight_cdk_macros::{
    algorithm_lens_finder, chainsight_common, did_export, init_in, lens_method,
};
use ic_web3_rs::futures::{future::BoxFuture, FutureExt};
mod app;
chainsight_common!(60);
init_in!();
mod chainlink_ethusd;
algorithm_lens_finder!("chainlink_ethusd", "proxy_get_last_snapshot_value", String);
lens_method!(u128, 1usize);
did_export!("aggregator_lens_ethusd");
