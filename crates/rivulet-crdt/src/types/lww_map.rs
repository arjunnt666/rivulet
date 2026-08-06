use rivulet_core::clock::Dot;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct Entry { value: Option<Value>, dot: Dot }

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct LwwMap { entries: HashMap<String, Entry> }

impl LwwMap {
    pub fn new() -> Self { Self::default() }

    pub fn set(&mut self, key: impl Into<String>, value: Value, dot: Dot) {
        let key = key.into();
        let replace = match self.entries.get(&key) { Some(e) => dot > e.dot, None => true };
        if replace { self.entries.insert(key, Entry { value: Some(value), dot }); }
    }

    pub fn delete(&mut self, key: &str, dot: Dot) {
        if let Some(e) = self.entries.get_mut(key) {
            if dot > e.dot { e.value = None; e.dot = dot; }
        }
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.entries.get(key).and_then(|e| e.value.as_ref())
    }
}
