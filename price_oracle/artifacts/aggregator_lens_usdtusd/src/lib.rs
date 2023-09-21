use candid::CandidType;
use chainsight_cdk::lens::LensFinder;
use chainsight_cdk_macros::{
    algorithm_lens_finder, chainsight_common, did_export, init_in, lens_method,
};
use ic_web3_rs::futures::{future::BoxFuture, FutureExt};
mod app;
chainsight_common!(60);
init_in!();
mod binance_usdtusd;
use crate::binance_usdtusd::SnapshotValue as SnapshotValue_0;
algorithm_lens_finder!(
    "binance_usdtusd",
    "proxy_get_last_snapshot_value",
    SnapshotValue_0
);
mod coingecko_usdtusd;
use crate::coingecko_usdtusd::SnapshotValue as SnapshotValue_1;
algorithm_lens_finder!(
    "coingecko_usdtusd",
    "proxy_get_last_snapshot_value",
    SnapshotValue_1
);
mod coinmarketcap_usdtusd;
use crate::coinmarketcap_usdtusd::SnapshotValue as SnapshotValue_2;
algorithm_lens_finder!(
    "coinmarketcap_usdtusd",
    "proxy_get_last_snapshot_value",
    SnapshotValue_2
);
lens_method!(u128, 3usize);
did_export!("aggregator_lens_usdtusd");
