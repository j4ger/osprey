#[cfg(feature = "server")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "server", derive(Serialize, Deserialize))]
pub struct ServerConfig {
    pub address: String,
    pub port: u16,
    pub queue_size: usize,
    pub logger_queue_size: usize,
    pub management_queue_size: usize,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            address: "0.0.0.0".to_string(),
            port: 8080,
            queue_size: 32,
            logger_queue_size: 32,
            management_queue_size: 32,
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "server", derive(Serialize, Deserialize))]
pub struct RssConfig {
    pub rsshub_url: String,
    pub default_interval: u64,
}

impl Default for RssConfig {
    fn default() -> Self {
        Self {
            rsshub_url: "https://rsshub.app/".to_string(),
            default_interval: 3600,
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "server", derive(Serialize, Deserialize))]
pub struct PushConfig {
    pub default_interval: u64,
}

impl Default for PushConfig {
    fn default() -> Self {
        Self {
            default_interval: 3600,
        }
    }
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "server", derive(Serialize, Deserialize))]
pub struct Config {
    pub server: ServerConfig,
    pub rss: RssConfig,
    pub push: PushConfig,
}
