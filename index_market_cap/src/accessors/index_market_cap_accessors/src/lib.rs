use index_market_cap_bindings as bindings ; algorithm_lens_finder ! ("batch_get_value_in_bulk_snapshot_indexer_https_push" , "proxy_batch_get_value" , bindings :: bulk_snapshot_indexer_https_push :: ResponseType , bindings :: bulk_snapshot_indexer_https_push :: RequestArgsType) ; use chainsight_cdk :: lens :: LensFinder ; use chainsight_cdk_macros :: algorithm_lens_finder ; async fn _get_target_proxy (target : candid :: Principal) -> candid :: Principal { let out : ic_cdk :: api :: call :: CallResult < (candid :: Principal ,) > = ic_cdk :: api :: call :: call (target , "get_proxy" , ()) . await ; out . unwrap () . 0 }