mod types ; use crate :: ic_web3_rs :: ethabi ; use ic_web3_rs ; pub type CallCanisterResponse = types :: ResponseType ; pub fn filter (_ : & CallCanisterResponse) -> bool { true }