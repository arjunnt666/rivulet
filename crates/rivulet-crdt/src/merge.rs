use crate::types::pn_counter::PnCounter;
use rivulet_core::clock::VersionVector;

pub fn merge_counters(a: &mut PnCounter, b: &PnCounter) { a.merge(b); }
pub fn merge_vv(a: &mut VersionVector, b: &VersionVector) { a.merge(b); }
