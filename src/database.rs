use surreal::prelude::*;
use std::sync::{Arc, RwLock};
use std::collections::HashMap;

struct Database {
    data: Arc<RwLock<HashMap<String, String>>>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}
//implementing the create method
impl Database {
    pub fn create(&self, key: String, value: String) -> Result<(), surreal::Error> {
        let mut data = self.data.write().unwrap();
        if data.contains_key(&key) {
            return Err(surreal::Error::new("Key already exists"));
        }
        data.insert(key, value);
        Ok(())
    }
}