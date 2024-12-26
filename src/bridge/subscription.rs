use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Subscription {
    pub id: usize,
    pub url: String,
    pub name: String,
    pub interval: u64,
    pub last_update: u64,
    pub push_targets: Vec<usize>,
    // TODO: template
}
