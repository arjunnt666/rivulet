use async_trait::async_trait;
use rivulet_core::document::{DocId, Document};
use rivulet_core::error::RivuletError;
use rivulet_core::op::Op;

#[async_trait]
pub trait Storage: Send + Sync {
    async fn load_doc(&self, id: &DocId) -> Result<Option<Document>, RivuletError>;
    async fn save_doc(&self, doc: &Document) -> Result<(), RivuletError>;
    async fn append_ops(&self, id: &DocId, ops: &[Op]) -> Result<(), RivuletError>;
    async fn list_docs(&self) -> Result<Vec<DocId>, RivuletError>;
}
