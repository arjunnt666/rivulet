//! Ephemeral presence: cursors, selections, "user is typing".

use rivulet_core::actor::ActorId;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PresenceState {
    pub actor: ActorId,
    pub cursor: Option<String>,
    pub selection: Option<(String, String)>,
    pub meta: Value,
    pub updated_ms: u64,
}

#[derive(Clone, Debug, Default)]
pub struct PresenceMap { states: HashMap<ActorId, PresenceState> }

impl PresenceMap {
    pub fn new() -> Self { Self::default() }
    pub fn upsert(&mut self, state: PresenceState) { self.states.insert(state.actor, state); }
    pub fn remove(&mut self, actor: &ActorId) { self.states.remove(actor); }
    pub fn list(&self) -> Vec<&PresenceState> { self.states.values().collect() }
}
