use rivulet_crdt::LwwMap;
use rivulet_core::error::RivuletError;
use serde_json::Value;

pub struct QueryEngine;

impl QueryEngine {
    pub fn new() -> Self { Self }
    pub fn get_path(&self, map: &LwwMap, path: &str) -> Result<Option<Value>, RivuletError> {
        Ok(map.get(path).cloned())
    }
    pub fn filter_keys(&self, map: &LwwMap, prefix: &str) -> Vec<String> {
        let _ = (map, prefix);
        vec![]
    }
}

impl Default for QueryEngine { fn default() -> Self { Self::new() } }
