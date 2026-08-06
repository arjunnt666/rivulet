use crate::actor::ActorId;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Dot {
    pub actor: ActorId,
    pub counter: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionVector {
    entries: BTreeMap<ActorId, u64>,
}

impl VersionVector {
    pub fn new() -> Self { Self::default() }

    pub fn get(&self, actor: &ActorId) -> u64 {
        self.entries.get(actor).copied().unwrap_or(0)
    }

    pub fn increment(&mut self, actor: ActorId) -> Dot {
        let next = self.get(&actor) + 1;
        self.entries.insert(actor, next);
        Dot { actor, counter: next }
    }

    pub fn observes(&self, dot: &Dot) -> bool {
        self.get(&dot.actor) >= dot.counter
    }

    pub fn merge(&mut self, other: &VersionVector) {
        for (actor, counter) in &other.entries {
            let cur = self.get(actor);
            if *counter > cur {
                self.entries.insert(*actor, *counter);
            }
        }
    }

    pub fn dominates(&self, other: &VersionVector) -> bool {
        other.entries.iter().all(|(a, c)| self.get(a) >= *c)
    }
}
