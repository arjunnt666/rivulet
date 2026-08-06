use rivulet_core::clock::VersionVector;
use rivulet_core::document::DocId;
use rivulet_core::op::Op;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SyncMessage {
    Hello { peer_vv: VersionVector },
    Request(SyncRequest),
    Response(SyncResponse),
    OpBroadcast { doc: DocId, op: Op },
    PresenceDelta { bytes: Vec<u8> },
    Goodbye,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SyncRequest { pub doc: DocId, pub have: VersionVector }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SyncResponse {
    pub doc: DocId,
    pub missing_ops: Vec<Op>,
    pub their_vv: VersionVector,
}
