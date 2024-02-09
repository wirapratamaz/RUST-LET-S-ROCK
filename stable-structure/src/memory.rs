// Import necessary modules and types from the `ic_stable_structures` crate.
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::DefaultMemoryImpl;
use std::cell::RefCell;

// Define constants for different memory IDs. These are unique identifiers for different
// memory areas that we want to manage.
const UPGRADES: MemoryId = MemoryId::new(0);
const STABLE_BTREE: MemoryId = MemoryId::new(1);

// Define the type alias `Memory` for the `VirtualMemory` that uses `DefaultMemoryImpl`.
// This makes it easier to refer to this specific type of virtual memory throughout the code.
pub type Memory = VirtualMemory<DefaultMemoryImpl>;

// Declare a thread-local variable `MEMORY_MANAGER`. This variable will be unique to each thread
// and will hold our memory manager instance. We use `RefCell` to allow for interior mutability,
// which means we can modify the memory manager even if we have a shared reference to it.
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
}

// Define a function to get the virtual memory instance for upgrades.
// This function uses the `MEMORY_MANAGER` to retrieve the memory associated with the `UPGRADES` ID.
pub fn get_upgrades_memory() -> Memory {
    MEMORY_MANAGER.with(|manager_ref| {
        // Borrow the memory manager immutably to get the memory instance.
        manager_ref.borrow().get(UPGRADES)
    })
}

// Define a function to get the virtual memory instance for the stable B-tree.
// This function is similar to `get_upgrades_memory` but retrieves the memory for the `STABLE_BTREE` ID.
pub fn get_stable_btree_memory() -> Memory {
    MEMORY_MANAGER.with(|manager_ref| {
        // Borrow the memory manager immutably to get the memory instance.
        manager_ref.borrow().get(STABLE_BTREE)
    })
}