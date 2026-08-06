use crate::traits::Storage;
use async_trait::async_trait;
use rivulet_core::document::{DocId, Document};
use rivulet_core::error::RivuletError;
use rivulet_core::op::Op;

pub struct SqliteStorage { pub path: String }

impl SqliteStorage {
    pub fn open(path: impl Into<String>) -> Self { Self { path: path.into() } }
}

#[async_trait]
impl Storage for SqliteStorage {
    async fn load_doc(&self, _id: &DocId) -> Result<Option<Document>, RivuletError> {
        Err(RivuletError::Storage("sqlite backend not linked in this build".into()))
    }
    async fn save_doc(&self, _doc: &Document) -> Result<(), RivuletError> {
        Err(RivuletError::Storage("sqlite backend not linked in this build".into()))
    }
    async fn append_ops(&self, _id: &DocId, _ops: &[Op]) -> Result<(), RivuletError> {
        Err(RivuletError::Storage("sqlite backend not linked in this build".into()))
    }
    async fn list_docs(&self) -> Result<Vec<DocId>, RivuletError> { Ok(vec![]) }
}
