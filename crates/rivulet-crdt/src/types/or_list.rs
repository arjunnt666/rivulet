use rivulet_core::clock::Dot;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Item { value: Value, dot: Dot, deleted: bool }

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct OrList {
    items: HashMap<String, Item>,
    order: Vec<String>,
}

impl OrList {
    pub fn new() -> Self { Self::default() }

    pub fn insert(&mut self, item_id: String, value: Value, dot: Dot, index_hint: usize) {
        self.items.insert(item_id.clone(), Item { value, dot, deleted: false });
        let idx = index_hint.min(self.order.len());
        self.order.insert(idx, item_id);
    }

    pub fn remove(&mut self, item_id: &str, _dot: Dot) {
        if let Some(item) = self.items.get_mut(item_id) { item.deleted = true; }
    }

    pub fn values(&self) -> Vec<&Value> {
        self.order.iter().filter_map(|id| {
            self.items.get(id).and_then(|i| if i.deleted { None } else { Some(&i.value) })
        }).collect()
    }
}
