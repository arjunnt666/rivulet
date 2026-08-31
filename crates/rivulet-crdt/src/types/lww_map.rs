use rivulet_core::clock::Dot;
use rivulet_core::op::{Op, OpPayload};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;


#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct Entry {
    value: Option<Value>,
    dot: Dot,
}

/// Last-writer-wins map. Ties broken by Dot ordering.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct LwwMap {
    entries: HashMap<String, Entry>,
}

impl LwwMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, key: impl Into<String>, value: Value, dot: Dot) {
        let key = key.into();
        let replace = match self.entries.get(&key) {
            Some(e) => dot > e.dot,
            None => true,
        };
        if replace {
            self.entries.insert(key, Entry { value: Some(value), dot });
        }
    }

    pub fn delete(&mut self, key: &str, dot: Dot) {
        if let Some(e) = self.entries.get_mut(key) {
            if dot > e.dot {
                e.value = None;
                e.dot = dot;
            }
        }
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.entries.get(key).and_then(|e| e.value.as_ref())
    }

    pub fn apply_op(&mut self, op: &Op) {
        match &op.payload {
            OpPayload::MapSet { key, value } => self.set(key.clone(), value.clone(), op.id.dot),
            OpPayload::MapDelete { key } => self.delete(key, op.id.dot),
            _ => {}
        }
    }

    pub fn from_ops(ops: &[Op]) -> Self {
        let mut map = Self::new();
        for op in ops {
            map.apply_op(op);
        }
        map
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use rivulet_core::actor::ActorId;
    use rivulet_core::clock::VersionVector;

    #[test]
    fn last_write_wins() {
        let actor = ActorId::new();
        let mut vv = VersionVector::new();
        let mut map = LwwMap::new();
        let d1 = vv.increment(actor);
        map.set("k", serde_json::json!("a"), d1);
        let d2 = vv.increment(actor);
        map.set("k", serde_json::json!("b"), d2);
        assert_eq!(map.get("k").unwrap(), &serde_json::json!("b"));
        map.set("k", serde_json::json!("old"), d1);
        assert_eq!(map.get("k").unwrap(), &serde_json::json!("b"));
    }

    #[test]
    fn two_peers_same_key_agree() {
        use rivulet_core::{push_pull, ActorId, Document, OpPayload};
        let alice = ActorId::new();
        let bob = ActorId::new();
        let mut a = Document::new();
        let mut b = Document::new();
        b.id = a.id;
        a.local_op(
            alice,
            OpPayload::MapSet {
                key: "title".into(),
                value: serde_json::json!("hello"),
            },
        );
        b.local_op(
            bob,
            OpPayload::MapSet {
                key: "title".into(),
                value: serde_json::json!("world"),
            },
        );
        push_pull(&mut a, &mut b);
        let ma = LwwMap::from_ops(&a.ops);
        let mb = LwwMap::from_ops(&b.ops);
        assert_eq!(ma.get("title"), mb.get("title"));
        assert!(ma.get("title").is_some());
    }
}

