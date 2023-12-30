use std::str::FromStr;

use ic_web3_rs::{ethabi::Address, types::U256};

mod types;
pub type CallCanisterResponse = types::ResponseType;

pub type LensArgs = proposal_relayer_scroll_sepolia_bindings::LensArgs;

thread_local! {
    // TODO: handle edge case when tx is not relayed
    static LAST_RELAYED: std::cell::RefCell<u64> = std::cell::RefCell::new(0);
}

pub fn call_args() -> (u64, u64) {
    LAST_RELAYED.with_borrow(|r| {
        let last_relayed = r.clone();
        (last_relayed + 1, u64::MAX)
    })
}
pub fn filter(res: &CallCanisterResponse) -> bool {
    if res.0.len() == 0 {
        return false;
    }
    let last_relayed = res.4.iter().max().unwrap().clone();
    LAST_RELAYED.with_borrow_mut(|r| {
        *r = (last_relayed as u64).clone();
    });
    true
}

pub struct ContractCallArgs {
    pub ids: Vec<U256>,
    pub proposers: Vec<Address>,
    pub chainIds: Vec<U256>,
    pub startTimestamps: Vec<U256>,
    pub endTimestamps: Vec<U256>,
    pub proposedBlocks: Vec<U256>,
}

pub fn convert(res: &CallCanisterResponse) -> ContractCallArgs {
    let ids = res
        .0
        .clone()
        .iter()
        .map(|x| U256::from(x.clone()))
        .collect::<Vec<U256>>();
    let proposers = res
        .1
        .clone()
        .iter()
        .map(|x| Address::from_str(x).unwrap())
        .collect::<Vec<Address>>();
    let chainIds = res
        .2
        .clone()
        .iter()
        .map(|x| U256::from(x.clone()))
        .collect::<Vec<U256>>();
    let startTimestamps = res
        .3
        .clone()
        .iter()
        .map(|x| U256::from(x.clone()))
        .collect::<Vec<U256>>();
    let endTimestamps = res
        .4
        .clone()
        .iter()
        .map(|x| U256::from(x.clone()))
        .collect::<Vec<U256>>();
    let proposedBlocks = res
        .5
        .clone()
        .iter()
        .map(|x| U256::from(x.clone()))
        .collect::<Vec<U256>>();
    ContractCallArgs {
        ids,
        proposers,
        chainIds,
        startTimestamps,
        endTimestamps,
        proposedBlocks,
    }
}
