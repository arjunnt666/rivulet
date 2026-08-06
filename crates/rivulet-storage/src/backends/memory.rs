use crate::traits::Storage;
use async_trait::async_trait;
use rivulet_core::document::{DocId, Document};
use rivulet_core::error::RivuletError;
use rivulet_core::op::Op;
use std::collections::HashMap;
use std::sync::Mutex;

pub struct MemoryStorage { docs: Mutex<HashMap<DocId, Document>> }

impl MemoryStorage {
    pub fn new() -> Self { Self { docs: Mutex::new(HashMap::new()) } }
}
impl Default for MemoryStorage { fn default() -> Self { Self::new() } }

#[async_trait]
impl Storage for MemoryStorage {
    async fn load_doc(&self, id: &DocId) -> Result<Option<Document>, RivuletError> {
        Ok(self.docs.lock().unwrap().get(id).cloned())
    }
    async fn save_doc(&self, doc: &Document) -> Result<(), RivuletError> {
        self.docs.lock().unwrap().insert(doc.id, doc.clone());
        Ok(())
    }
    async fn append_ops(&self, id: &DocId, ops: &[Op]) -> Result<(), RivuletError> {
        let mut guard = self.docs.lock().unwrap();
        if let Some(doc) = guard.get_mut(id) {
            for op in ops { doc.apply(op.clone()); }
        }
        Ok(())
    }
    async fn list_docs(&self) -> Result<Vec<DocId>, RivuletError> {
        Ok(self.docs.lock().unwrap().keys().copied().collect())
    }
}
