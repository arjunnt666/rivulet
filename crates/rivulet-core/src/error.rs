use thiserror::Error;

#[derive(Debug, Error)]
pub enum RivuletError {
    #[error("causal dependency missing for op")]
    MissingDependency,
    #[error("document not found")]
    DocNotFound,
    #[error("storage error: {0}")]
    Storage(String),
    #[error("sync protocol error: {0}")]
    Sync(String),
    #[error("crypto error: {0}")]
    Crypto(String),
    #[error("query error: {0}")]
    Query(String),
}
