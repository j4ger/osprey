#[cfg(feature = "server")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "server", derive(Serialize, Deserialize))]
pub struct Subscription {
    pub id: usize,
    pub url: String,
    pub name: String,
    pub interval: u64,
    pub last_update: u64,
    pub push_targets: Vec<usize>,
    // TODO: template
}
