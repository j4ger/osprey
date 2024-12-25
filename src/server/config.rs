use anyhow::Context;
use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};

use crate::bridge::config::Config;

const STORE_PATH: &'static str = "osprey.toml";

impl Config {
    pub fn load() -> Result<Self, figment::Error> {
        Figment::from(Serialized::defaults(Config::default()))
            .merge(Toml::file(STORE_PATH))
            .merge(Env::prefixed("OSPREY_"))
            .extract()
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let toml = toml::to_string(self).context("Failed to serialize config.")?;
        std::fs::write(STORE_PATH, toml).context("Failed to write config to disk.")?;
        Ok(())
    }
}
