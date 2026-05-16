mod hash_map;
mod linked_list;

use hash_map::HashMap;
use linked_list::DoublyLinkedList;
use std::hash::Hash;

pub struct LruCache<K, V> {
    map: HashMap<K>,
    list: DoublyLinkedList<K, V>,
    capacity: usize,
}

impl<K: Hash + Eq + Clone, V> LruCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "LruCache capacity must be greater than zero");

        Self {
            map: HashMap::new(),
            list: DoublyLinkedList::new(),
            capacity,
        }
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}