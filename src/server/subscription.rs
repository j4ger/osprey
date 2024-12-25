use anyhow::Context;
use chrono::DateTime;
use figment::{
    providers::{Format, Toml},
    Figment,
};
use rss::{Channel, Item};
use tokio::sync::{broadcast, mpsc, oneshot};
use tracing::warn;

use crate::bridge::subscription::Subscription;

use super::message::{StatMessage, UpdateMessage};

const STORE_PATH: &'static str = "subscriptions.toml";

pub fn load_subscriptions() -> Result<Vec<Subscription>, figment::Error> {
    if !std::path::Path::new(STORE_PATH).exists() {
        return Ok(Vec::new());
    }
    Figment::new().merge(Toml::file(STORE_PATH)).extract()
}

pub fn save_subscriptions(subscriptions: &[Subscription]) -> anyhow::Result<()> {
    let toml = toml::to_string(subscriptions).context("Failed to serialize subscriptions.")?;
    std::fs::write("subscriptions.toml", toml).context("Failed to write subscriptions to disk.")?;
    Ok(())
}

impl Subscription {
    pub async fn fetch(&mut self) -> anyhow::Result<Option<Vec<Item>>> {
        use anyhow::Context;
        let mut result = Vec::new();
        let content = reqwest::get(&self.url)
            .await
            .context("Failed to send request.")?
            .bytes()
            .await
            .context("Failed to decode bytes from response.")?;
        let channel =
            Channel::read_from(&content[..]).context("Failed to parse RSS channel data.")?;
        for item in channel.items() {
            if let Some(pub_time) = item.pub_date() {
                if let Ok(pub_time) = DateTime::parse_from_rfc2822(pub_time) {
                    if pub_time.timestamp() as u64 > self.last_update {
                        self.last_update = pub_time.timestamp() as u64;
                        result.push(item.clone());
                    }
                } else {
                    warn!("Failed to parse datetime: {}", pub_time);
                    continue;
                }
            } else {
                continue;
            }
        }
        if result.is_empty() {
            Ok(None)
        } else {
            Ok(Some(result))
        }
    }
}

pub async fn handle_subscription(
    subscription: Subscription,
    sender: broadcast::Sender<UpdateMessage>,
    mut terminator: oneshot::Receiver<()>,
    stat_sender: mpsc::Sender<StatMessage>,
) {
    async fn check_update(
        subscription: &mut Subscription,
        sender: &broadcast::Sender<UpdateMessage>,
        stat_sender: &mpsc::Sender<StatMessage>,
    ) {
        if let Some(items) = subscription.fetch().await.unwrap() {
            for item in items {
                if let Some(update_time) = item.pub_date() {
                    if let Ok(update_time) = chrono::DateTime::parse_from_rfc2822(update_time) {
                        if update_time.timestamp() as u64 > subscription.last_update {
                            subscription.last_update = update_time.timestamp() as u64;
                        } else {
                            continue;
                        }
                    }
                } else {
                    continue;
                }
                let message = UpdateMessage {
                    item,
                    targets: subscription.push_targets.clone(),
                };
                // TODO: error handling
                sender.send(message).unwrap();
            }
        }
    }
    let mut subscription = subscription.clone();
    check_update(&mut subscription, &sender, &stat_sender).await;
    loop {
        tokio::select! {
            _ = &mut terminator => break,
            _ = tokio::time::sleep(tokio::time::Duration::from_secs(subscription.interval)) => {
                check_update(&mut subscription, &sender, &stat_sender).await;
            },
        }
    }
}
