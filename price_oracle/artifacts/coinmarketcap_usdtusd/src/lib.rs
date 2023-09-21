use chainsight_cdk :: web2 :: { JsonRpcSnapshotParam , Web2JsonRpcSnapshotIndexer } ; use chainsight_cdk_macros :: { chainsight_common , did_export , init_in , prepare_stable_structure , stable_memory_for_vec , timer_task_func , StableMemoryStorable , } ; use candid :: { Decode , Encode } ; init_in ! () ; chainsight_common ! (60) ; mod app ; use app :: * ; # [derive (Debug , Clone , candid :: CandidType , candid :: Deserialize , serde :: Serialize , StableMemoryStorable)] # [stable_mem_storable_opts (max_size = 10000 , is_fixed_size = false)] pub struct Snapshot { pub value : SnapshotValue , pub timestamp : u64 , } prepare_stable_structure ! () ; stable_memory_for_vec ! ("snapshot" , Snapshot , 0 , true) ; timer_task_func ! ("set_task" , "index" , true) ; async fn index () { let indexer = Web2JsonRpcSnapshotIndexer :: new ("https://pro-api.coinmarketcap.com/v2/cryptocurrency/quotes/latest" . to_string () ,) ; let res = indexer . get :: < String , SnapshotValue > (JsonRpcSnapshotParam { headers : vec ! [("X-CMC_PRO_API_KEY" . to_string () , "449b4e6b-48b0-4a59-849a-71b76d6c23de" . to_string ()) , ("Content-Type" . to_string () , "application/json" . to_string ()) ,] . into_iter () . collect () , queries : vec ! [("slug" . to_string () , "oasys" . to_string ()) ,] . into_iter () . collect () , }) . await . unwrap () ; let snapshot = Snapshot { value : res , timestamp : ic_cdk :: api :: time () / 1000000 , } ; let _ = add_snapshot (snapshot . clone ()) ; } fn _get_last_snapshot_value () -> SnapshotValue { get_last_snapshot () . value } fn _get_top_snapshot_values (n : u64) -> Vec < SnapshotValue > { get_top_snapshots (n) . iter () . map (| s | s . value . clone ()) . collect () } fn _get_snapshot_value (idx : u64) -> SnapshotValue { get_snapshot (idx) . value } # [ic_cdk :: query] # [candid :: candid_method (query)] pub fn get_last_snapshot_value () -> SnapshotValue { _get_last_snapshot_value () } # [ic_cdk :: query] # [candid :: candid_method (query)] pub fn get_top_snapshot_values (n : u64) -> Vec < SnapshotValue > { _get_top_snapshot_values (n) } # [ic_cdk :: query] # [candid :: candid_method (query)] pub fn get_snapshot_value (idx : u64) -> SnapshotValue { _get_snapshot_value (idx) } # [ic_cdk :: update] # [candid :: candid_method (update)] pub async fn proxy_get_last_snapshot_value (input : std :: vec :: Vec < u8 >) -> std :: vec :: Vec < u8 > { use chainsight_cdk :: rpc :: Receiver ; chainsight_cdk :: rpc :: ReceiverProviderWithoutArgs :: < SnapshotValue > :: new (proxy () , _get_last_snapshot_value ,) . reply (input) . await } # [ic_cdk :: update] # [candid :: candid_method (update)] pub async fn proxy_get_top_snapshot_values (input : std :: vec :: Vec < u8 >) -> std :: vec :: Vec < u8 > { use chainsight_cdk :: rpc :: Receiver ; chainsight_cdk :: rpc :: ReceiverProvider :: < u64 , Vec < SnapshotValue >> :: new (proxy () , _get_top_snapshot_values ,) . reply (input) . await } # [ic_cdk :: update] # [candid :: candid_method (update)] pub async fn proxy_get_snapshot_value (input : std :: vec :: Vec < u8 >) -> std :: vec :: Vec < u8 > { use chainsight_cdk :: rpc :: Receiver ; chainsight_cdk :: rpc :: ReceiverProvider :: < u64 , SnapshotValue > :: new (proxy () , _get_snapshot_value ,) . reply (input) . await } did_export ! ("coinmarketcap_oasusd") ;