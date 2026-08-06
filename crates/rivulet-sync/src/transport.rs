use crate::protocol::SyncMessage;
use async_trait::async_trait;
use rivulet_core::error::RivuletError;

#[async_trait]
pub trait Transport: Send + Sync {
    async fn send(&self, msg: SyncMessage) -> Result<(), RivuletError>;
    async fn recv(&self) -> Result<SyncMessage, RivuletError>;
}

pub struct Loopback;

#[async_trait]
impl Transport for Loopback {
    async fn send(&self, _msg: SyncMessage) -> Result<(), RivuletError> { Ok(()) }
    async fn recv(&self) -> Result<SyncMessage, RivuletError> {
        Err(RivuletError::Sync("loopback empty".into()))
    }
}
