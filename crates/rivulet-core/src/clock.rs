use crate::actor::ActorId;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// A single (actor, counter) pair. The atom of causality.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct Dot {
    pub actor: ActorId,
    pub counter: u64,
}

/// Version vector. Compact summary of "what I've seen".
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionVector {
    entries: BTreeMap<ActorId, u64>,
}

impl VersionVector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, actor: &ActorId) -> u64 {
        self.entries.get(actor).copied().unwrap_or(0)
    }

    pub fn increment(&mut self, actor: ActorId) -> Dot {
        let next = self.get(&actor) + 1;
        self.entries.insert(actor, next);
        Dot { actor, counter: next }
    }

    pub fn observe(&mut self, dot: Dot) {
        let cur = self.get(&dot.actor);
        if dot.counter > cur {
            self.entries.insert(dot.actor, dot.counter);
        }
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

    /// True if self dominates other (we've seen everything they have, and maybe more).
    pub fn dominates(&self, other: &VersionVector) -> bool {
        other.entries.iter().all(|(a, c)| self.get(a) >= *c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actor::ActorId;

    #[test]
    fn increment_and_observe() {
        let actor = ActorId::new();
        let mut vv = VersionVector::new();
        let d = vv.increment(actor);
        assert!(vv.observes(&d));
        assert_eq!(d.counter, 1);
    }

    #[test]
    fn merge_takes_max() {
        let a = ActorId::new();
        let mut left = VersionVector::new();
        let mut right = VersionVector::new();
        left.increment(a);
        right.increment(a);
        right.increment(a);
        left.merge(&right);
        assert_eq!(left.get(&a), 2);
    }

    #[test]
    fn observe_remote_dot() {
        let a = ActorId::new();
        let mut remote = VersionVector::new();
        let d = remote.increment(a);
        remote.increment(a);
        let mut local = VersionVector::new();
        local.observe(d);
        assert!(local.observes(&d));
        assert_eq!(local.get(&a), 1);
    }
}
