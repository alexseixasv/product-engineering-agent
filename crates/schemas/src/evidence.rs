use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Evidence {
    pub path: String,
    pub start_line: u32,
    pub end_line: u32,
    pub excerpt: String,
}

impl Evidence {
    pub fn validate(&self) -> Result<(), String> {
        if self.path.trim().is_empty() {
            return Err("evidence.path must be non-empty".into());
        }
        if self.excerpt.trim().is_empty() {
            return Err("evidence.excerpt must be non-empty".into());
        }
        if self.start_line == 0 || self.end_line == 0 {
            return Err("evidence line numbers are 1-indexed".into());
        }
        if self.end_line < self.start_line {
            return Err("evidence.end_line must be >= start_line".into());
        }
        Ok(())
    }
}
