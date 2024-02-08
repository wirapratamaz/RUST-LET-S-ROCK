use ic_stable_structure::memory_manager::{MemoryManager, MemoryId, VirtualMemory};
use ic_stable_structure::DefaultMemoryImpl;
use std::cell::RefCell;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
struct State {
    // Equals to `true` while the [build_index] task runs.
    is_build_index_running: bool,

    /// The principal of the ledger canister that is indexed by this index.
    ledger_id: Principal,

    /// The maximum number of transactions returned by [get_blocks].
    max_blocks_per_response: u64,
}

// Memory for upgrades, where data from the heap can be serialized and deserialized
const UPGRADE_MEMORY: MemoryId = MemoryId::new(0);

// Memory for the stable memory manager using StableBTreeMap
const STABLE_MEMORY: MemoryId = MemoryId::new(1);

pub type MemoryManager = VirtualMemory<DefaultMemoryImpl>;

// The memory manager is used for simulating multiple memories. Given a `MemoryId` it can
// return a memory that can be used by stable structures.
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl> = RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
}

pub fn with_memory_manager<F, R>(f: F) -> R
where
    F: FnOnce(&MemoryManager<DefaultMemoryImpl>) -> R,
{
    MEMORY_MANAGER.with(|memory_manager| f(&memory_manager.borrow()))
}

pub fn with_memory_manager_mut<F, R>(f: F) -> R
where
    F: FnOnce(&mut MemoryManager<DefaultMemoryImpl>) -> R,
{
    MEMORY_MANAGER.with(|memory_manager| f(&mut memory_manager.borrow_mut()))
}

pub fn with_upgrade_memory<F, R>(f: F) -> R
where
    F: FnOnce(&MemoryManager<DefaultMemoryImpl>) -> R,
{
    with_memory_manager(|memory_manager| {
        memory_manager.with_memory(UPGRADE_MEMORY, f)
    })
}

pub fn get_upgrades_memory() -> MemoryManager<DefaultMemoryImpl> {
    with_memory_manager(|memory_manager| memory_manager.get_memory(UPGRADE_MEMORY))
}

pub fn get_stable_btree_memory() -> MemoryManager<DefaultMemoryImpl> {
    with_memory_manager(|memory_manager| memory_manager.get_memory(STABLE_MEMORY))
}

pub fn with_stable_btree_memory<F, R>(f: F) -> R
where
    F: FnOnce(&MemoryManager<DefaultMemoryImpl>) -> R,
{
    with_memory_manager(|memory_manager| {
        memory_manager.with_memory(STABLE_MEMORY, f)
    })
}

pub fn with_blocks<R>(f: impl FnOnce(&mut MemoryManager<DefaultMemoryImpl>) -> R) -> R {
    with_memory_manager_mut(|memory_manager| f(memory_manager))
}

fn with_state<R>(f: impl FnOnce(&State) -> R) -> R {
    STATE
        .with(|cell| f(&*cell.borrow()))
        .expect("Failed to access state")
}

fn change_state(f:impl FnOnce(&mut State)){
    STATE
        .with(|cell| {
            let mut borrowed = cell.borrow_mut();
            let mut state = *borrowed.get();
            f(&mut state);
            borrowed.set(state);
        })
        .expect("Failed to change state");
}