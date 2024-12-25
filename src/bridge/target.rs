#[cfg(feature = "server")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "server", derive(Serialize, Deserialize))]
pub struct Target {
    pub id: usize,
    pub name: String,
    pub url: String,
    pub interval: u64,
}
