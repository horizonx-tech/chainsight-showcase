use rpl_eth_0_3_indexer_lens_accessors :: * ;
pub type LensValue = u64;
pub async fn calculate (targets : Vec < String >) -> LensValue {
  let _result = get_get_last_snapshot_value_in_rpl_eth_0_3_indexer (targets . get (0usize) . unwrap () . clone ()) . await ;
  let edpr = _result.unwrap().edpr;
  let edpr_int = (edpr * 10000.0) as u64;
  edpr_int
}
