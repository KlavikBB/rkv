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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_missing() {
        let store = Store::new();
        assert_eq!(store.get("asd"), None);
    }

    #[test]
    fn test_set() {
        let mut store = Store::new();
        store.set("asd", "sda");
        assert_eq!(store.get("asd"), Some("sda"));
    }

    #[test]
    fn test_second_set() {
        let mut store = Store::new();
        store.set("asd", "sda");
        store.set("asd", "dsa");
        assert_eq!(store.get("asd"), Some("dsa"));
    }

    #[test]
    fn test_del() {
        let mut store = Store::new();
        store.set("asd", "sda");
        assert_eq!(store.del("asd"), true);
        assert_eq!(store.get("asd"), None);
    }

    #[test]
    fn test_del_missing() {
        let mut store = Store::new();
        assert_eq!(store.del("asd"), false);
    }

    #[test]
    fn test_exists() {
        let mut store = Store::new();
        store.set("asd", "sda");
        assert_eq!(store.exists("asd"), true);
    }

    #[test]
    fn test_exists_missing() {
        let store = Store::new();
        assert_eq!(store.exists("asd"), false);
    }
}
