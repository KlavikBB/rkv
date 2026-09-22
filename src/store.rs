use std::collections::HashMap;

pub struct Store {
    hashmap: HashMap<String, String>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            hashmap: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.hashmap.insert(key.to_string(), value.to_string());
    }
    pub fn get(&self, key: &str) -> Option<&str> {
        self.hashmap.get(key).map(|s| s.as_str())
    }
    pub fn del(&mut self, key: &str) -> bool {
        self.hashmap.remove(key).is_some()
    }
    pub fn exists(&self, key: &str) -> bool {
        self.hashmap.contains_key(key)
    }
}
