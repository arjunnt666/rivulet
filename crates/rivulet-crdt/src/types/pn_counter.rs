use rivulet_core::actor::ActorId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PnCounter {
    p: HashMap<ActorId, u64>,
    n: HashMap<ActorId, u64>,
}

impl PnCounter {
    pub fn new() -> Self { Self::default() }

    pub fn add(&mut self, actor: ActorId, delta: i64) {
        if delta >= 0 { *self.p.entry(actor).or_default() += delta as u64; }
        else { *self.n.entry(actor).or_default() += (-delta) as u64; }
    }

    pub fn value(&self) -> i64 {
        let pos: u64 = self.p.values().sum();
        let neg: u64 = self.n.values().sum();
        pos as i64 - neg as i64
    }

    pub fn merge(&mut self, other: &PnCounter) {
        for (a, v) in &other.p { let cur = self.p.entry(*a).or_default(); *cur = (*cur).max(*v); }
        for (a, v) in &other.n { let cur = self.n.entry(*a).or_default(); *cur = (*cur).max(*v); }
    }
}
