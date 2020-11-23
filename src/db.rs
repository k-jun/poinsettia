use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct DB {
    store: Arc<Mutex<HashMap<String, String>>>,
}

impl DB {
    pub fn new() -> DB {
        let hp = HashMap::new();
        DB {
            store: Arc::new(Mutex::new(hp)),
        }
    }
    pub fn set(&self, key: String, value: String) -> String {
        let mut store = self.store.lock().unwrap();
        match store.insert(key, value) {
            Some(s) => s.to_string(),
            None => String::new(),
        }
    }

    pub fn get(&self, key: String) -> String {
        let store = self.store.lock().unwrap();
        match store.get(&key) {
            Some(s) => s.to_string(),
            None => String::new(),
        }
    }
}

#[test]
fn test_new() {
    let _ = DB::new();
}

#[test]
fn test_set() {
    struct TestCase {
        in_key: String,
        in_value: String,
    }

    let cases = vec![
        TestCase {
            in_key: "labore".to_string(),
            in_value: "id".to_string(),
        },
        TestCase {
            in_key: "ut".to_string(),
            in_value: "commodi".to_string(),
        },
    ];

    for c in cases {
        let db = DB::new();
        db.set(c.in_key, c.in_value);
    }
}

#[test]
fn test_get() {
    struct TestCase {
        current_key: String,
        current_value: String,
        in_key: String,
        out_value: String,
    }

    let cases = vec![
        TestCase {
            current_key: "consequuntur".to_string(),
            current_value: "nihil".to_string(),
            in_key: "consequuntur".to_string(),
            out_value: "nihil".to_string(),
        },
        TestCase {
            current_key: "ea".to_string(),
            current_value: "consequatur".to_string(),
            in_key: "ea".to_string(),
            out_value: "consequatur".to_string(),
        },
    ];

    for c in cases {
        // let db = DB::new();
        let mut hp = HashMap::new();
        hp.insert(c.current_key, c.current_value);
        let db = DB {
            store: Arc::new(Mutex::new(hp)),
        };
        let value = db.get(c.in_key);
        assert_eq!(c.out_value, value);
    }
}
