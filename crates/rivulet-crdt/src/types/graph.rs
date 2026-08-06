use rivulet_core::clock::Dot;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CrdtGraph {
    nodes: HashMap<String, Dot>,
    edges: HashMap<(String, String), Dot>,
    tombstone_nodes: HashSet<String>,
    tombstone_edges: HashSet<(String, String)>,
}

impl CrdtGraph {
    pub fn new() -> Self { Self::default() }
    pub fn add_node(&mut self, id: String, dot: Dot) { self.nodes.insert(id, dot); }
    pub fn remove_node(&mut self, id: &str) { self.tombstone_nodes.insert(id.to_string()); }
    pub fn add_edge(&mut self, from: String, to: String, dot: Dot) { self.edges.insert((from, to), dot); }
    pub fn nodes(&self) -> Vec<&String> {
        self.nodes.keys().filter(|n| !self.tombstone_nodes.contains(*n)).collect()
    }
}
