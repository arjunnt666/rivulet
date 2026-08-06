//! Optional relay — catch-up cache, not source of truth.

use rivulet_core::document::DocId;
use rivulet_core::op::Op;
use std::collections::HashMap;

pub struct RelayState { buffers: HashMap<DocId, Vec<Op>> }

impl RelayState {
    pub fn new() -> Self { Self { buffers: HashMap::new() } }
    pub fn push(&mut self, doc: DocId, op: Op) { self.buffers.entry(doc).or_default().push(op); }
    pub fn take_since(&mut self, doc: &DocId, _after_len: usize) -> Vec<Op> {
        self.buffers.get(doc).cloned().unwrap_or_default()
    }
}

impl Default for RelayState { fn default() -> Self { Self::new() } }
