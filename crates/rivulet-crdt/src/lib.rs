//! CRDT zoo.
//!
//! Not inventing new math — just wiring the usual suspects so apps can
//! stop hand-rolling last-write-wins maps at 2am.

pub mod types;
pub mod apply;
pub mod merge;

pub use types::lww_map::LwwMap;
pub use types::rga_text::RgaText;
pub use types::or_list::OrList;
pub use types::pn_counter::PnCounter;
pub use types::graph::CrdtGraph;
