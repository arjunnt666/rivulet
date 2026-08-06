use crate::actor::ActorId;
use crate::clock::VersionVector;
use crate::op::Op;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DocId(Uuid);

impl DocId {
    pub fn new() -> Self { Self(Uuid::new_v4()) }
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
        Self { id: DocId::new(), vv: VersionVector::new(), ops: Vec::new(), actors_seen: Vec::new() }
    }

    pub fn apply(&mut self, op: Op) {
        if !self.vv.observes(&op.id.dot) {
            self.vv.increment(op.actor);
            self.ops.push(op);
        }
    }
}

impl Default for Document {
    fn default() -> Self { Self::new() }
}
