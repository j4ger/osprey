use anyhow::Context;
use figment::{
    providers::{Format, Serialized, Toml},
    Figment,
};
use tokio::{
    select,
    sync::{broadcast, mpsc, oneshot},
};

use crate::bridge::target::Target;

use super::message::{StatMessage, UpdateMessage};

const STORE_PATH: &'static str = "targets.toml";

pub fn load_targets() -> Result<Vec<Target>, figment::Error> {
    if !std::path::Path::new(STORE_PATH).exists() {
        return Ok(Vec::new());
    }
    Figment::from(Serialized::defaults(Vec::<Target>::new()))
        .merge(Toml::file(STORE_PATH))
        .extract()
}

pub fn save_targets(targets: &[Target]) -> anyhow::Result<()> {
    let toml = toml::to_string(targets).context("Failed to serialize targets.")?;
    std::fs::write("targets.toml", toml).context("Failed to write targets to disk.")?;
    Ok(())
}

pub async fn handle_target(
    target: Target,
    mut recv: broadcast::Receiver<UpdateMessage>,
    mut terminator: oneshot::Receiver<()>,
    stat_sender: mpsc::Sender<StatMessage>,
) {
    // await for the terminator signal and broadcast, log out each update message for now
    loop {
        select! {
           _ = &mut terminator => { break; }
           message = recv.recv() => {
                let message = if let Err(err) = message {
                    // log
                    continue;
                } else {
                    message.unwrap()
                };
                if !message.targets.contains(&target.id) {
                    continue;
                }
                println!("recv: {}", message.item.title.unwrap());
           }
        }
    }
}
