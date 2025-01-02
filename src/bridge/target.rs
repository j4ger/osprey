use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Target {
    pub id: usize,
    pub name: String,
    pub url: String,
    pub interval: u64,
}

impl Target {
    pub fn empty() -> Self {
        Self {
            id: 0,
            name: String::new(),
            url: String::new(),
            interval: 0,
        }
    }
}
