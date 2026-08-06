use crate::actor::ActorId;
use crate::clock::Dot;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OpId { pub dot: Dot }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OpPayload {
    MapSet { key: String, value: Value },
    MapDelete { key: String },
    TextInsert { pos: String, content: String },
    TextDelete { start: String, end: String },
    ListInsert { index_hint: u64, item_id: String, value: Value },
    ListRemove { item_id: String },
    CounterAdd { delta: i64 },
    Extension { type_id: String, body: Vec<u8> },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Op {
    pub id: OpId,
    pub parent_vv: crate::clock::VersionVector,
    pub payload: OpPayload,
    pub actor: ActorId,
}
