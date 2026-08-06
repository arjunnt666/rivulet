use crate::protocol::*;
use rivulet_core::clock::VersionVector;
use rivulet_core::document::{DocId, Document};
use rivulet_core::error::RivuletError;
use tracing::debug;

pub struct SyncSession {
    pub local_vv: VersionVector,
    pub remote_vv: VersionVector,
}

impl SyncSession {
    pub fn new() -> Self {
        Self { local_vv: VersionVector::new(), remote_vv: VersionVector::new() }
    }

    pub fn on_hello(&mut self, remote: VersionVector) {
        self.remote_vv.merge(&remote);
        debug!("session hello merged remote vv");
    }

    pub fn build_request(&self, doc: DocId) -> SyncRequest {
        SyncRequest { doc, have: self.local_vv.clone() }
    }

    pub fn apply_response(&mut self, doc: &mut Document, resp: SyncResponse) -> Result<usize, RivuletError> {
        let mut applied = 0;
        for op in resp.missing_ops { doc.apply(op); applied += 1; }
        self.remote_vv.merge(&resp.their_vv);
        self.local_vv.merge(&doc.vv);
        Ok(applied)
    }
}

impl Default for SyncSession {
    fn default() -> Self { Self::new() }
}
