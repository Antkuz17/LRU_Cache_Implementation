use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const INITIAL_BUCKETS: usize = 16;

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
}