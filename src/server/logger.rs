use tokio::{
    select,
    sync::{mpsc, oneshot},
};

use super::message::StatMessage;

// TODO: proper logging

pub async fn handle_log(
    mut rx: mpsc::Receiver<StatMessage>,
    mut terminator: oneshot::Receiver<()>,
) {
    loop {
        select! {
            _ = &mut terminator => { break; }
            message = rx.recv() => {
                let message = if let None = message {
                    // log
                    continue;
                } else {
                    message.unwrap()
                };
                match message {
                    StatMessage::TargetSuccess { id } => {
                        println!("Target {} succeeded.", id);
                    }
                    StatMessage::TargetFailure { id } => {
                        println!("Target {} failed.", id);
                    }
                    StatMessage::SubscriptionSuccess { id, time } => {
                        println!("Subscription {} succeeded at {}.", id, time);
                    }
                    StatMessage::SubscriptionFailure { id } => {
                        println!("Subscription {} failed.", id);
                    }
                }
            }
        }
    }
}
