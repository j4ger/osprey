use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Target {
    pub id: usize,
    pub name: String,
    pub url: String,
    pub interval: u64,
}
