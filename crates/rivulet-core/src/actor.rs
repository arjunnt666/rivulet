use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Opaque identity for a replica / peer / tab.
/// Not a user id — one user can have many actors (phone + laptop + that one tab you forgot).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ActorId(Uuid);

impl ActorId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_bytes(bytes: [u8; 16]) -> Self {
        Self(Uuid::from_bytes(bytes))
    }
}

impl Default for ActorId {
    fn default() -> Self {
        Self::new()
    }
}
