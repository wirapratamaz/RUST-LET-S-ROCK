use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;  

// Dummy implementation for traits and function
pub trait Storable {
    fn to_bytes(&self) -> Cow<[u8]>;
    fn from_bytes(bytes: &[u8]) -> Self;
}

pub struct MemoryId {
    id: u32,
}

fn serialize_then_unwrap<T: Serialize>(value: &T) -> Vec<u8> {
    bincode::serialize(value).unwrap()
}

fn deserialize_then_unwrap<T: for<'de> Deserialize<'de>>(bytes: &[u8]) -> T {
    bincode::deserialize(bytes).unwrap()
}

fn get_orders_log_index_memory() -> impl Memory{
    MEMORY_MANAGER.with(|manager_ref| {
        manager_ref.borrow().get(ORDERS_LOG_INDEX)
    })
}

fn get_orders_log_data_memory() -> impl Memory{
    MEMORY_MANAGER.with(|manager_ref| {
        manager_ref.borrow().get(ORDERS_LOG_DATA)
    })
}

// Define StableLog struct
pub struct StableLog<T, Index, Data> 
where
    T: Storable,
    Index: Storable,
    Data: Storable,
{
    _marker: PhantomData<T>,
    index_memory: Memory,
    data_memory: Memory,
}

impl<T, Index, Data> StableLog<T, Index, Data>
where
    T: Storable,
    Index: Storable,
    Data: Storable,
{
    pub fn new(index_memory: Memory, data_memory: Memory) -> Self {
        Self {
            _marker: PhantomData,
            index_memory,
            data_memory,
        }
    }

    pub fn append(&self, value: T) {
        let index = self.index_memory.get::<Index>(0);
        let data = self.data_memory.get::<Data>(0);
        let index_bytes = index.to_bytes();
        let data_bytes = data.to_bytes();
        let value_bytes = value.to_bytes();
        let new_index = index + 1;
        self.index_memory.set(0, new_index);
        self.data_memory.set(new_index, value_bytes);
    }

    pub fn get(&self, index: Index) -> Option<T> {
        let index = index.to_bytes();
        let data = self.data_memory.get::<Data>(index);
        Some(data)
    }

    pub fn init (index_memory: Memory, data_memory: Memory) -> Self {
        Self::new(index_memory, data_memory)
    }

    pub fn len(&self) -> u64 {
        let index = self.index_memory.get::<Index>(0);
        index
    }
}

//Define the LogEntry and Action types
#[derive(Serialize, Deserialize)]
pub struct LogEntry {
    timestamp: u64,
    exchange_id: u64,
    action: Action,
}

#[derive(Serialize, Deserialize)]
enum Action {
    OrderMade,
    OrderCancelled,
}

// implement storable for LogEntry
impl Storable for LogEntry {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(serialize_then_unwrap(self))
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        deserialize_then_unwrap(bytes)
    }
}

// implement display for LogEntry
impl Display for LogEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "LogEntry: timestamp: {}, exchange_id: {}, action: {:?}", self.timestamp, self.exchange_id, self.action)
    }
}