use crate::actor::ActorId;
use crate::clock::VersionVector;
use crate::op::{Op, OpId, OpPayload};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DocId(Uuid);

impl DocId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for DocId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Document {
    pub id: DocId,
    pub vv: VersionVector,
    pub ops: Vec<Op>,
    pub actors_seen: Vec<ActorId>,
}

impl Document {
    pub fn new() -> Self {
        Self {
            id: DocId::new(),
            vv: VersionVector::new(),
            ops: Vec::new(),
            actors_seen: Vec::new(),
        }
    }

    pub fn local_op(&mut self, actor: ActorId, payload: OpPayload) -> Op {
        let parent_vv = self.vv.clone();
        let dot = self.vv.increment(actor);
        if !self.actors_seen.contains(&actor) {
            self.actors_seen.push(actor);
        }
        let op = Op {
            id: OpId { dot },
            parent_vv,
            payload,
            actor,
        };
        self.ops.push(op.clone());
        op
    }

    pub fn apply(&mut self, op: Op) {
        if self.vv.observes(&op.id.dot) {
            return;
        }
        self.vv.observe(op.id.dot);
        if !self.actors_seen.contains(&op.actor) {
            self.actors_seen.push(op.actor);
        }
        self.ops.push(op);
    }

    pub fn ops_not_in(&self, have: &VersionVector) -> Vec<Op> {
        self.ops
            .iter()
            .filter(|op| !have.observes(&op.id.dot))
            .cloned()
            .collect()
    }
}

/// Two peers exchange missing ops. Last write is whatever apply order does
/// at the document layer; CRDT merge happens above this.
pub fn push_pull(a: &mut Document, b: &mut Document) {
    let to_b = a.ops_not_in(&b.vv);
    let to_a = b.ops_not_in(&a.vv);
    for op in to_b {
        b.apply(op);
    }
    for op in to_a {
        a.apply(op);
    }
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actor::ActorId;
    use serde_json::json;

    #[test]
    fn two_peers_converge() {
        let alice = ActorId::new();
        let bob = ActorId::new();
        let mut a = Document::new();
        let mut b = Document::new();
        a.id = b.id;
        a.local_op(
            alice,
            OpPayload::MapSet {
                key: "title".into(),
                value: json!("hello"),
            },
        );
        b.local_op(
            bob,
            OpPayload::MapSet {
                key: "body".into(),
                value: json!("world"),
            },
        );
        push_pull(&mut a, &mut b);
        assert_eq!(a.ops.len(), 2);
        assert_eq!(b.ops.len(), 2);
        assert!(a.vv.dominates(&b.vv) && b.vv.dominates(&a.vv));
    }
}
