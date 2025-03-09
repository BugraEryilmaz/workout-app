use linked_hash_map::LinkedHashMap;

#[derive(Clone)]
pub struct LruCache<K: std::hash::Hash + Eq, V> {
    map: LinkedHashMap<K, V>,
    capacity: usize,
}

impl<K: std::hash::Hash + Eq + Clone, V> LruCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        Self {
            map: LinkedHashMap::new(),
            capacity,
        }
    }

    pub fn get_without_update(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        // Move accessed item to the most recently used position
        if let Some(value) = self.map.remove(key) {
            self.map.insert(key.clone(), value);
        }
        self.map.get(key)
    }

    pub fn put(&mut self, key: K, value: V) {
        if self.map.contains_key(&key) {
            self.map.remove(&key); // Remove existing key to update position
        } else if self.map.len() >= self.capacity {
            self.map.pop_front(); // Evict the least recently used item
        }
        self.map.insert(key, value);
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.map.iter().rev()
    }
}
