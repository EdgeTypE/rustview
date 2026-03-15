/// Global computation cache for `#[cached]` macro.
///
/// Cache is per-process, shared across sessions.
/// Keyed on function name + FNV-1a hash of serialized arguments.
/// LRU eviction with configurable max entries per function (default: 128).
use dashmap::DashMap;
use std::any::Any;
use std::collections::VecDeque;
use std::sync::Mutex;

/// Default maximum cache entries per function.
const DEFAULT_MAX_ENTRIES: usize = 128;

/// A single cache entry storing the result.
struct CacheEntry {
    value: Box<dyn Any + Send + Sync>,
}

/// LRU cache for a single function.
struct FunctionCache {
    entries: std::collections::HashMap<u64, CacheEntry>,
    order: VecDeque<u64>,
    max_entries: usize,
}

impl FunctionCache {
    fn new(max_entries: usize) -> Self {
        FunctionCache {
            entries: std::collections::HashMap::new(),
            order: VecDeque::new(),
            max_entries,
        }
    }

    fn get<T: Clone + 'static>(&mut self, key: u64) -> Option<T> {
        if self.entries.contains_key(&key) {
            // Move to back (most recently used)
            self.order.retain(|&k| k != key);
            self.order.push_back(key);
            self.entries
                .get(&key)
                .and_then(|e| e.value.downcast_ref::<T>())
                .cloned()
        } else {
            None
        }
    }

    fn insert<T: Clone + Send + Sync + 'static>(&mut self, key: u64, value: T) {
        // Evict LRU if at capacity
        if self.entries.len() >= self.max_entries && !self.entries.contains_key(&key) {
            if let Some(evicted_key) = self.order.pop_front() {
                self.entries.remove(&evicted_key);
            }
        }

        self.order.retain(|&k| k != key);
        self.order.push_back(key);
        self.entries.insert(
            key,
            CacheEntry {
                value: Box::new(value),
            },
        );
    }

    fn clear(&mut self) {
        self.entries.clear();
        self.order.clear();
    }
}

/// Global cache store.
static GLOBAL_CACHE: once_cell::sync::Lazy<DashMap<String, Mutex<FunctionCache>>> =
    once_cell::sync::Lazy::new(DashMap::new);

/// Get a cached value for a function.
pub fn get_cached<T: Clone + 'static>(fn_name: &str, key: u64) -> Option<T> {
    let entry = GLOBAL_CACHE.get(fn_name)?;
    let mut cache = entry.lock().ok()?;
    cache.get::<T>(key)
}

/// Insert a value into the cache for a function.
pub fn insert_cached<T: Clone + Send + Sync + 'static>(fn_name: &str, key: u64, value: T) {
    GLOBAL_CACHE
        .entry(fn_name.to_string())
        .or_insert_with(|| Mutex::new(FunctionCache::new(DEFAULT_MAX_ENTRIES)));
    if let Some(entry) = GLOBAL_CACHE.get(fn_name) {
        if let Ok(mut cache) = entry.lock() {
            cache.insert(key, value);
        }
    }
}

/// Clear all caches (called on hot-reload).
pub fn clear_all_caches() {
    for entry in GLOBAL_CACHE.iter() {
        if let Ok(mut cache) = entry.value().lock() {
            cache.clear();
        }
    }
}

/// Clear cache for a specific function.
pub fn clear_function_cache(fn_name: &str) {
    if let Some(entry) = GLOBAL_CACHE.get(fn_name) {
        if let Ok(mut cache) = entry.lock() {
            cache.clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Use unique function names per test to avoid cross-test interference
    // in the global cache.

    #[test]
    fn test_cache_miss_returns_none() {
        let result = get_cached::<i64>("test_miss_fn", 12345);
        assert!(result.is_none());
    }

    #[test]
    fn test_cache_hit() {
        insert_cached("test_hit_fn", 100, 42i64);
        let result = get_cached::<i64>("test_hit_fn", 100);
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_cache_different_keys() {
        insert_cached("test_diffkeys_fn", 1, "hello".to_string());
        insert_cached("test_diffkeys_fn", 2, "world".to_string());

        assert_eq!(
            get_cached::<String>("test_diffkeys_fn", 1),
            Some("hello".to_string())
        );
        assert_eq!(
            get_cached::<String>("test_diffkeys_fn", 2),
            Some("world".to_string())
        );
    }

    #[test]
    fn test_cache_clear() {
        insert_cached("test_clear_fn", 1, 10i64);
        clear_function_cache("test_clear_fn");
        assert!(get_cached::<i64>("test_clear_fn", 1).is_none());
    }

    #[test]
    fn test_cache_lru_eviction() {
        let fn_name = "test_lru_fn";
        // Insert DEFAULT_MAX_ENTRIES + 1 entries
        for i in 0..=DEFAULT_MAX_ENTRIES as u64 {
            insert_cached(fn_name, i, i as i64);
        }
        // The first entry (key=0) should have been evicted
        assert!(get_cached::<i64>(fn_name, 0).is_none());
        // The last entry should still be there
        assert_eq!(
            get_cached::<i64>(fn_name, DEFAULT_MAX_ENTRIES as u64),
            Some(DEFAULT_MAX_ENTRIES as i64)
        );
    }

    #[test]
    fn test_clear_all_caches() {
        insert_cached("test_clearall_fn1", 1, 1i64);
        insert_cached("test_clearall_fn2", 1, 2i64);
        clear_all_caches();
        assert!(get_cached::<i64>("test_clearall_fn1", 1).is_none());
        assert!(get_cached::<i64>("test_clearall_fn2", 1).is_none());
    }
}
