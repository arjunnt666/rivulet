//! Core primitives: actor ids, version vectors, ops, and the document shell.

pub mod actor;
pub mod clock;
pub mod document;
pub mod error;
pub mod op;

pub use actor::ActorId;
pub use clock::{Dot, VersionVector};
pub use document::{DocId, Document};
pub use error::RivuletError;
pub use op::{Op, OpId, OpPayload};
