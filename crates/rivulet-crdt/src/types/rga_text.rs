use rivulet_core::clock::Dot;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RgaText {
    chars: Vec<(String, char, Dot, bool)>,
}

impl RgaText {
    pub fn new() -> Self { Self::default() }

    pub fn insert(&mut self, pos: String, content: &str, dot: Dot) {
        for (i, ch) in content.chars().enumerate() {
            let id = format!("{pos}:{i}");
            self.chars.push((id, ch, dot, false));
        }
    }

    pub fn delete_range(&mut self, start: &str, end: &str) {
        let mut active = false;
        for (id, _, _, deleted) in &mut self.chars {
            if id == start { active = true; }
            if active { *deleted = true; }
            if id == end { break; }
        }
    }

    pub fn to_string(&self) -> String {
        self.chars.iter().filter(|(_, _, _, d)| !*d).map(|(_, c, _, _)| c).collect()
    }
}
