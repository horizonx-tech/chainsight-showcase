mod types;
use std::str::FromStr;

use crate::ic_web3_rs::ethabi;
use ic_web3_rs;
use ic_web3_rs::{ethabi::Address, types::U256};
pub type CallCanisterResponse = types::ResponseType;
pub type LensArgs = voting_relayer_sepolia_bindings::LensArgs;

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
#[derive(Clone, Debug)]
pub struct ContractCallArgs {
    pub ids: Vec<U256>,
    pub voters: Vec<Address>,
    pub _supports: Vec<bool>,
    pub votingPowers: Vec<U256>,
    pub chainIds: Vec<U256>,
}
impl ContractCallArgs {
    pub fn new(
        ids: Vec<U256>,
        voters: Vec<ethabi::Address>,
        _supports: Vec<bool>,
        voting_powers: Vec<U256>,
        chain_ids: Vec<U256>,
    ) -> Self {
        Self {
            ids,
            voters,
            _supports,
            votingPowers: voting_powers,
            chainIds: chain_ids,
        }
    }
}
pub fn convert(res: &CallCanisterResponse) -> ContractCallArgs {
    let ids = res
        .0
        .clone()
        .iter()
        .map(|x| U256::from(x.clone()))
        .collect::<Vec<ic_web3_rs::types::U256>>();
    let voters = res
        .1
        .clone()
        .iter()
        .map(|x| Address::from_str(x).unwrap())
        .collect::<Vec<Address>>();
    let _supports = res.2.clone();
    let voting_powers = res
        .3
        .clone()
        .iter()
        .map(|x| U256::from(x.clone()))
        .collect::<Vec<ic_web3_rs::types::U256>>();
    let chain_ids = res
        .4
        .clone()
        .iter()
        .map(|x| U256::from(x.clone()))
        .collect::<Vec<ic_web3_rs::types::U256>>();
    ContractCallArgs::new(ids, voters, _supports, voting_powers, chain_ids)
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
