use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const INITIAL_BUCKETS: usize = 256;

pub struct HashMap<K> {
    buckets: Vec<Vec<(K, usize)>>,
    size: usize,
}

impl<K: Hash + Eq> HashMap<K> {
    pub fn new() -> Self {
        let mut buckets = Vec::with_capacity(INITIAL_BUCKETS);
        for _ in 0..INITIAL_BUCKETS {
            buckets.push(Vec::new());
        }

        Self {
            buckets,
            size: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn bucket_index(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.buckets.len()
    }


    pub fn insert(&mut self, key: K, value: usize) -> Option<usize> {
        let index = self.bucket_index(&key);
        let bucket = &mut self.buckets[index];

        for entry in bucket.iter_mut() {
            if entry.0 == key {
                let old = entry.1;
                entry.1 = value;
                return Some(old);
            }
        }

        bucket.push((key, value));
        self.size += 1;
        None
    }

    pub fn get(&self, key: &K) -> Option<usize> {
        let index = self.bucket_index(key);
        let bucket = &self.buckets[index];

        for entry in bucket.iter() {
            if entry.0 == *key {
                return Some(entry.1);
            }
        }

        None
    }

    pub fn remove(&mut self, key: &K) -> Option<usize> {
        let index = self.bucket_index(key);
        let bucket = &mut self.buckets[index];

        for i in 0..bucket.len() {
            if bucket[i].0 == *key {
                let (_, value) = bucket.swap_remove(i);
                self.size -= 1;
                return Some(value);
            }
        }

        None
    }
}