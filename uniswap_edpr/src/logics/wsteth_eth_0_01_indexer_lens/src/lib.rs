use wsteth_eth_0_01_indexer_lens_accessors :: * ;
# [derive (Clone , Debug , Default , candid :: CandidType , serde :: Deserialize , serde :: Serialize)]
pub type LensValue = u64;
pub async fn calculate (targets : Vec < String >) -> LensValue {
  let _result = get_get_last_snapshot_in_wsteth_eth_0_01_indexer (targets . get (0usize) . unwrap () . clone ()) . await ;
  let edpr = _result.unwrap().edpr;
  let edpr_int = (edpr * 10000.0) as u64;
  edpr_int
}
