use std::cell::RefCell;

use candid::ser::IDLBuilder;
use candid::{Decode, Encode, Principal};
use chainsight_cdk::rpc::AsyncReceiverProvider;
use chainsight_cdk::rpc::Receiver;
use chainsight_cdk::rpc::ReceiverProvider;
use chainsight_cdk::rpc::ReceiverProviderWithoutArgs;
use chainsight_cdk_macros::{
    chainsight_common, did_export, init_in, prepare_stable_structure, stable_memory_for_scalar,
    timer_task_func, StableMemoryStorable,
};

use ic_cdk::api::call::call_raw;
use ic_stable_structures::{memory_manager::MemoryId, BTreeMap};
use ic_web3_rs::futures::future::join_all;
use ic_web3_rs::futures::future::BoxFuture;
use ic_web3_rs::futures::FutureExt;
use index_indexer_bindings::{LensArgs, Sources};
use index_market_cap::LensValue;
use types::{Key, QueryOption, Snapshot, SnapshotId, SnapshotValue, Task, Value};
use ulid_lib::Ulid;

mod types;
chainsight_common!();
init_in!(2);
timer_task_func!("set_task", "index", 6);
prepare_stable_structure!();

macro_rules! only_proxy {
    () => {
        if ic_cdk::api::caller() != get_proxy() {
            ic_cdk::trap("Unauthorized");
        }
    };
}

macro_rules! only_controller {
    () => {
        if !ic_cdk::api::is_controller(&ic_cdk::api::caller()) {
            ic_cdk::trap("Unauthorized");
        }
    };
}
thread_local! {
    static SNAPSHOTS: RefCell<BTreeMap<SnapshotId, Snapshot, MemoryType>> =  RefCell::new(
        BTreeMap::init(
            MEMORY_MANAGER.with(|mm|mm.borrow().get(MemoryId::new(6)))
        )
    );

    static TASKS: RefCell<BTreeMap<Key, Task, MemoryType>> = RefCell::new(
        BTreeMap::init(
            MEMORY_MANAGER.with(|mm|mm.borrow().get(MemoryId::new(7)))
        )
    );
    static SNAPSHOT_IDS: RefCell<ic_stable_structures::Vec<SnapshotId,MemoryType>> = RefCell::new(
        ic_stable_structures::Vec::init(
            MEMORY_MANAGER.with(|mm|mm.borrow().get(MemoryId::new(8)))).unwrap()
    );
    static CONFIGS: RefCell<BTreeMap<Key,Value,MemoryType>> = RefCell::new(
        BTreeMap::init(
            MEMORY_MANAGER.with(|mm|mm.borrow().get(MemoryId::new(9)))
        )
    );
}

#[ic_cdk::update]
#[candid::candid_method(update)]
fn setup() -> Result<(), String> {
    Ok(())
}

#[ic_cdk::update]
#[candid::candid_method(update)]
fn add_task(task: Task) {
    only_controller!();
    TASKS.with(|tasks| {
        tasks.borrow_mut().insert(
            Key {
                id: task.id.clone(),
            },
            task,
        );
    });
}

#[ic_cdk::update]
#[candid::candid_method(update)]
fn remove_task(id: String) {
    only_controller!();
    TASKS.with(|tasks| {
        tasks.borrow_mut().remove(&Key { id });
    });
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn tasks() -> Vec<Task> {
    let mut results = vec![];
    TASKS.with(|tasks| {
        for task in tasks.borrow().iter() {
            results.push(task.1.clone());
        }
    });
    results
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn max_count() -> u64 {
    CONFIGS.with(|configs| {
        let max_count = configs.borrow().get(&Key::from("max_count".to_string()));
        if max_count.is_none() {
            return 1000;
        }
        let max_count = max_count.unwrap();
        let max_count = max_count.value.parse::<u64>();
        if max_count.is_err() {
            return 1000;
        }
        max_count.unwrap()
    })
}

#[ic_cdk::update]
#[candid::candid_method(update)]
fn update_max_count(count: u64) {
    only_controller!();
    _update_max_count(count);
}

fn _update_max_count(count: u64) {
    CONFIGS.with(|configs| {
        configs.borrow_mut().insert(
            Key::from("max_count".to_string()),
            Value {
                value: count.to_string(),
            },
        );
    });
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn call_args() -> LensArgs {
    LensArgs {
        args: vec![],
        targets: vec![],
    }
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_last_snapshot() -> Snapshot {
    SNAPSHOTS.with(|snapshots| snapshots.borrow().last_key_value().unwrap().1.clone())
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_last_snapshot_value() -> SnapshotValue {
    get_last_snapshot().value
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_snapshots() -> Vec<Snapshot> {
    // unsupported
    vec![]
}

#[ic_cdk::query]
#[candid::candid_method(query)]
async fn query_between(opt: QueryOption) -> Vec<Snapshot> {
    _query_between(opt).await
}

#[ic_cdk::query]
#[candid::candid_method(query)]
async fn query_between_with(key: String, opt: QueryOption) -> Vec<(SnapshotId, f64)> {
    _query_between_with((key, opt)).await
}

#[ic_cdk::update]
#[candid::candid_method(update)]
async fn proxy_query_between_with(input: Vec<u8>) -> Vec<u8> {
    AsyncReceiverProvider::<(String, QueryOption), Vec<(SnapshotId, f64)>>::new(
        proxy(),
        _query_between_with,
    )
    .reply(input)
    .await
}

fn _query_between_with(input: (String, QueryOption)) -> BoxFuture<'static, Vec<(SnapshotId, f64)>> {
    async move {
        _query_between(input.1)
            .await
            .into_iter()
            .filter(|s| s.value.contains_key(&input.0))
            .map(|s| (s.id, s.value.get(&input.0).unwrap().to_owned()))
            .collect()
    }
    .boxed()
}

fn _query_between(opt: QueryOption) -> BoxFuture<'static, Vec<Snapshot>> {
    let from = opt.from_timestamp.unwrap_or(0);
    let to = opt.to_timestamp.unwrap_or(0);
    let divisor = 1_000_000; // nanosec to msec
    async move { snapshots_between((from / divisor) as u64, (to / divisor) as u64).await }.boxed()
}

async fn snapshots_between(from: u64, to: u64) -> Vec<Snapshot> {
    let mut result = vec![];
    let mut ids_to_fetch = vec![];
    SNAPSHOT_IDS.with(|id| {
        let snapshot_len = id.borrow().len();
        for idx in 0..snapshot_len {
            let id = id.borrow().get(snapshot_len - idx - 1).unwrap();
            let ulid = Ulid::from_string(&id.id).unwrap();
            if ulid.timestamp_ms().lt(&from) {
                break;
            }
            if ulid.timestamp_ms().le(&to) {
                ids_to_fetch.push(ulid);
            }
        }
        for id in ids_to_fetch {
            let snapshot = SNAPSHOTS
                .with(|snapshots| snapshots.borrow().get(&SnapshotId { id: id.to_string() }));
            if let Some(snapshot) = snapshot {
                result.push(snapshot.clone());
            }
        }
    });
    result
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_sources() -> Vec<Sources> {
    vec![]
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_snapshot(_: u64) -> Snapshot {
    // unsupported
    get_last_snapshot()
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_snapshot_value(_: u64) -> SnapshotValue {
    // unsupported
    get_last_snapshot_value()
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_value(key: String) -> Option<f64> {
    _get_value(key)
}

#[ic_cdk::update]
#[candid::candid_method(update)]
async fn proxy_get_value(key: Vec<u8>) -> Vec<u8> {
    ReceiverProvider::<String, Option<f64>>::new(proxy(), _get_value)
        .reply(key)
        .await
}

fn _get_value(key: String) -> Option<f64> {
    get_last_snapshot().value.get(&key).map(|v| *v)
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn snapshots_len() -> u64 {
    SNAPSHOT_IDS.with(|id| id.borrow().len() as u64)
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_top_snapshots(top: u64) -> Vec<Snapshot> {
    let mut result = vec![];
    let mut ids_to_fetch = vec![];
    SNAPSHOT_IDS.with(|id| {
        for idx in id.borrow().len()..0 {
            let id = id.borrow().get(idx).unwrap();
            ids_to_fetch.push(id);
            if ids_to_fetch.len() >= top as usize {
                break;
            }
        }
    });
    for id in ids_to_fetch {
        let snapshot =
            SNAPSHOTS.with(|snapshots| snapshots.borrow().get(&SnapshotId { id: id.id }));
        if let Some(snapshot) = snapshot {
            result.push(snapshot);
        }
    }
    result
}
#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_top_snapshot_values(top: u64) -> Vec<SnapshotValue> {
    get_top_snapshots(top)
        .into_iter()
        .map(|s| s.value)
        .collect()
}

fn _delete_snapshots() {
    let max_count = max_count();
    let len = snapshots_len();
    if len <= max_count {
        return;
    }
    _delete((len - max_count) as usize);
}

fn _delete(size: usize) {
    if size == 0 {
        return;
    }
    SNAPSHOT_IDS.with(|ids| {
        let ids = ids.borrow_mut();
        let new_ids = ids
            .iter()
            .skip(size)
            .map(|id| id.clone())
            .collect::<Vec<SnapshotId>>();
        let mut deletion_ids = vec![];
        for id in ids.iter().take(size) {
            deletion_ids.push(id.clone());
        }
        let mut idx = 0;
        for id in new_ids {
            ids.set(idx, &id);
            idx += 1;
        }
        let deletion_count = ids.len() - idx;
        for _ in 0..deletion_count {
            ids.pop();
        }
        SNAPSHOTS.with(|snapshots| {
            for id in deletion_ids {
                snapshots.borrow_mut().remove(&id);
            }
        });
    });
}

fn _add_snapshot(snapshot: Snapshot) {
    SNAPSHOTS.with(|snapshots| {
        snapshots
            .borrow_mut()
            .insert(snapshot.id.clone(), snapshot.clone());
    });
    SNAPSHOT_IDS.with(|ids| {
        ids.borrow_mut().push(&snapshot.id.clone()).unwrap();
    });
    _delete_snapshots();
}

async fn _index() -> Result<Snapshot, String> {
    let mut futures = vec![];
    let mut task_ids = vec![];
    TASKS.with(|tasks| {
        for (key, task) in tasks.borrow().iter() {
            task_ids.push(key.clone());
            let future = async move {
                let out = call(task.lens, task.to_lens_args()).await.unwrap();
                (task.id, out)
            };
            futures.push(future);
        }
    });
    let results = join_all(futures).await;
    let snapshot = Snapshot::from(results).await;
    _add_snapshot(snapshot.clone());
    Ok(snapshot)
}

#[ic_cdk::update]
#[candid::candid_method(update)]
async fn index() {
    only_proxy!();
    _index().await.unwrap();
}

#[ic_cdk::update]
#[candid::candid_method(update)]
async fn test_index() {
    _index().await;
}

async fn call(lens: Principal, args: LensArgs) -> Result<LensValue, String> {
    let idl: Vec<u8> = IDLBuilder::new()
        .arg(&args)
        .map_err(|e| format!("failed to encode args: {:?}", e))?
        .serialize_to_vec()
        .map_err(|e| format!("failed to serialize args: {:?}", e))?;
    let result = call_raw(lens, "get_result", idl, 0)
        .await
        .map_err(|e| format!("{:?}", e))?;
    Ok(Decode!(result.as_slice(), LensValue).unwrap())
    //    let method_name = "proxy_get_result";
    //    let px = _get_target_proxy(lens).await;
    //let result = CallProvider::new()
    //    .call(
    //        Message::new::<CallCanisterArgs>(args, px.clone(), &method_name)
    //            .map_err(|e| format!("failed to encode message: {:?}", e))?,
    //    )
    //    .await
    //    .map_err(|e| format!("failed to call: {:?}", e))?;
    //result
    //    .reply::<LensValue>()
    //    .map_err(|e| format!("failed to decode reply: {:?}", e))
}

/// proxy methods
#[ic_cdk::update]
#[candid::candid_method(update)]
async fn proxy_get_last_snapshot(input: Vec<u8>) -> Vec<u8> {
    ReceiverProviderWithoutArgs::<Snapshot>::new(proxy(), get_last_snapshot)
        .reply(input)
        .await
}

#[ic_cdk::update]
#[candid::candid_method(update)]
async fn proxy_get_last_snapshot_value(input: Vec<u8>) -> Vec<u8> {
    ReceiverProviderWithoutArgs::<SnapshotValue>::new(proxy(), get_last_snapshot_value)
        .reply(input)
        .await
}

#[ic_cdk::update]
#[candid::candid_method(update)]
async fn proxy_get_snapshot(input: Vec<u8>) -> Vec<u8> {
    ReceiverProvider::<u64, Snapshot>::new(proxy(), get_snapshot)
        .reply(input)
        .await
}

#[ic_cdk::update]
#[candid::candid_method(update)]
async fn proxy_get_snapshot_value(input: Vec<u8>) -> Vec<u8> {
    ReceiverProvider::<u64, SnapshotValue>::new(proxy(), get_snapshot_value)
        .reply(input)
        .await
}

#[ic_cdk::update]
#[candid::candid_method(update)]
async fn proxy_get_snapshots(input: Vec<u8>) -> Vec<u8> {
    ReceiverProviderWithoutArgs::<Vec<Snapshot>>::new(proxy(), get_snapshots)
        .reply(input)
        .await
}

#[ic_cdk::update]
#[candid::candid_method(update)]
async fn proxy_get_top_snapshot_values(input: Vec<u8>) -> Vec<u8> {
    ReceiverProvider::<u64, Vec<SnapshotValue>>::new(proxy(), get_top_snapshot_values)
        .reply(input)
        .await
}

#[ic_cdk::update]
#[candid::candid_method(update)]
async fn proxy_get_top_snapshots(input: Vec<u8>) -> Vec<u8> {
    ReceiverProvider::<u64, Vec<Snapshot>>::new(proxy(), get_top_snapshots)
        .reply(input)
        .await
}

#[ic_cdk::update]
#[candid::candid_method(update)]
async fn proxy_snapshots_len(input: Vec<u8>) -> Vec<u8> {
    ReceiverProviderWithoutArgs::<u64>::new(proxy(), snapshots_len)
        .reply(input)
        .await
}

#[ic_cdk::update]
#[candid::candid_method(update)]
async fn proxy_query_between(input: Vec<u8>) -> Vec<u8> {
    AsyncReceiverProvider::<QueryOption, Vec<Snapshot>>::new(proxy(), _query_between)
        .reply(input)
        .await
}

#[cfg(test)]
mod test2 {

    use std::collections::HashMap;

    use super::*;
    #[test]
    fn test_add_snapshot() {
        let snapshot = Snapshot {
            id: SnapshotId {
                id: "01HX8MP06M000000000000004F".to_string(),
            },
            value: HashMap::new(),
        };
        _add_snapshot(snapshot.clone());
        let result = SNAPSHOTS.with(|snapshots| snapshots.borrow().get(&snapshot.id));
        assert_eq!(result.unwrap().id, snapshot.id);
        let ids_result = SNAPSHOT_IDS.with(|ids| ids.borrow().len());
        assert_eq!(ids_result, 1);
    }
}

did_export!("rating_indexer");
