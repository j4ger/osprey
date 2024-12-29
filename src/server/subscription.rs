use anyhow::Context;
use figment::{
    providers::{Format, Toml},
    Figment,
};
use rss::{Channel, Item};
use time::{format_description::well_known::Rfc2822, PrimitiveDateTime};
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
                if let Ok(pub_time) = PrimitiveDateTime::parse(pub_time, &Rfc2822) {
                    let timestamp = pub_time.assume_utc().unix_timestamp();
                    if timestamp > self.last_update {
                        self.last_update = timestamp;
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
        stat_sender: &mpsc::Sender<StatMessage>, // TODO: let stat sender count the number of updates and last update time
                                                 // TODO: and write it to disk
    ) {
        if let Some(items) = subscription.fetch().await.unwrap() {
            for item in items {
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
