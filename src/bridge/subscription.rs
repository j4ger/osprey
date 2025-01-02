use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Subscription {
    pub id: usize,
    pub url: String,
    pub name: String,
    pub interval: u64,
    pub last_update: i64,
    pub push_targets: Vec<usize>,
    pub update_count: usize,
    // TODO: template
}

impl Subscription {
    pub fn empty() -> Self {
        Self {
            id: 0,
            url: String::new(),
            name: String::new(),
            interval: 0,
            last_update: 0,
            push_targets: Vec::new(),
            update_count: 0,
        }
    }
}
