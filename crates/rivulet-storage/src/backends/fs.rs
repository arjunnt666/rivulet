use crate::traits::Storage;
use async_trait::async_trait;
use rivulet_core::document::{DocId, Document};
use rivulet_core::error::RivuletError;
use rivulet_core::op::Op;
use std::path::PathBuf;

pub struct FsStorage { root: PathBuf }

impl FsStorage {
    pub fn new(root: impl Into<PathBuf>) -> Self { Self { root: root.into() } }
}

#[async_trait]
impl Storage for FsStorage {
    async fn load_doc(&self, _id: &DocId) -> Result<Option<Document>, RivuletError> { Ok(None) }
    async fn save_doc(&self, _doc: &Document) -> Result<(), RivuletError> { Ok(()) }
    async fn append_ops(&self, _id: &DocId, _ops: &[Op]) -> Result<(), RivuletError> { Ok(()) }
    async fn list_docs(&self) -> Result<Vec<DocId>, RivuletError> { Ok(vec![]) }
}
