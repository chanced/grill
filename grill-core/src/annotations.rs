use crate::Value;
use dashmap::DashMap;
use parking_lot::Mutex;
use std::{borrow::Borrow, sync::Arc};

#[derive(Clone)]
pub struct Annotations {
    pub valid: Arc<Mutex<bool>>,
    pub values: Arc<DashMap<String, Arc<Value>>>,
}
impl Annotations {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn valid(&self) -> bool {
        *self.valid.lock()
    }
    pub fn set_valid(&self, valid: bool) {
        *self.valid.lock() = valid;
    }
    pub fn get(&self, key: impl Borrow<str>) -> Option<Arc<Value>> {
        self.values.get(key.borrow()).map(|v| v.clone())
    }
    pub fn set(&self, key: impl ToString, value: Arc<Value>) {
        self.values.insert(key.to_string(), value);
    }
}

impl Default for Annotations {
    fn default() -> Self {
        Annotations {
            valid: Arc::new(Mutex::new(true)),
            values: Arc::new(DashMap::new()),
        }
    }
}

#[cfg(test)]
mod test {
    use std::ops::Deref;

    use super::*;
    #[test]
    fn test_insert() {
        let a = Annotations::new();
        let x = Arc::new(Value::String("test".to_string()));
        a.set("key", x);
        let r = a.get("key").unwrap();
        if let Value::String(s) = r.deref() {
            assert_eq!(s, "test");
        } else {
            panic!("not a string");
        }
        let o = Arc::new(Value::String("overwritten".to_string()));
        a.set("key", o);
        let r = a.get("key").unwrap();
        if let Value::String(s) = r.deref() {
            assert_eq!(s, "overwritten");
        } else {
            panic!("not a string");
        }
    }
}
