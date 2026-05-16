use lru_cache::LruCache;

fn main() {
    let mut cache: LruCache<String, i32> = LruCache::new(3);

    cache.put("a".to_string(), 1);
    cache.put("b".to_string(), 2);
    cache.put("c".to_string(), 3);

    println!("After inserting a, b, c:");
    println!("  len = {}", cache.len());
    println!("  get(a) = {:?}", cache.get(&"a".to_string()));

    cache.put("d".to_string(), 4);

    println!("\nAfter inserting d (cache was full):");
    println!("  len = {}", cache.len());
    println!("  get(a) = {:?}  (should still exist, we just accessed it)", cache.get(&"a".to_string()));
    println!("  get(b) = {:?}  (should be None, evicted as LRU)", cache.get(&"b".to_string()));
    println!("  get(c) = {:?}", cache.get(&"c".to_string()));
    println!("  get(d) = {:?}", cache.get(&"d".to_string()));
}