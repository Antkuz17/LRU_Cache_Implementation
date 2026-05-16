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

    pub fn get(&mut self, key: &K) -> Option<&V> {
        let index = self.map.get(key)?;
        self.list.move_to_front(index);
        self.list.get_value(index)
    }

    pub fn put(&mut self, key: K, value: V) {
        if let Some(index) = self.map.get(&key) {
            self.list.set_value(index, value);
            self.list.move_to_front(index);
            return;
        }

        if self.list.len() >= self.capacity {
            if let Some((evicted_key, _)) = self.list.pop_back() {
                self.map.remove(&evicted_key);
            }
        }

        let new_index = self.list.push_front(key.clone(), value);
        self.map.insert(key, new_index);
    }


}