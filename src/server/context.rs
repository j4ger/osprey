use dashmap::DashMap;
use tokio::sync::{mpsc, oneshot};

use crate::bridge::{config::Config, subscription::Subscription, target::Target};

use super::{
    management::{handle_management, CallResult, ManagementMessage, ManagementMessageCapsule},
    task::Task,
};

#[derive(Clone)]
pub struct AppContext {
    management_tx: mpsc::Sender<ManagementMessageCapsule>,
    targets: DashMap<usize, Target>,
    subscriptions: DashMap<usize, Subscription>,
}

impl AppContext {
    pub async fn new(
        config: Config,
        targets: Vec<Target>,
        subscriptions: Vec<Subscription>,
    ) -> (Self, Task) {
        let target_map: DashMap<usize, Target> = targets
            .iter()
            .map(|target| (target.id, target.clone()))
            .collect();
        let subscription_map: DashMap<usize, Subscription> = subscriptions
            .iter()
            .map(|subscription| (subscription.id, subscription.clone()))
            .collect();

        let (management_tx, management_rx) = mpsc::channel(config.server.management_queue_size);
        let (terminator_tx, terminator_rx) = oneshot::channel();
        let handle = tokio::spawn(handle_management(
            management_rx,
            terminator_rx,
            config,
            targets,
            subscriptions,
        ));

        let management_task = Task::new(terminator_tx, handle);

        let context = Self {
            management_tx,
            targets: target_map,
            subscriptions: subscription_map,
        };

        (context, management_task)
    }

    pub async fn call(&self, message: ManagementMessage) -> CallResult {
        let (tx, rx) = oneshot::channel();
        self.management_tx
            .send(ManagementMessageCapsule {
                message,
                response: tx,
            })
            .await
            .unwrap();
        rx.await.unwrap()
    }

    pub fn get_targets(&self) -> Vec<Target> {
        self.targets.iter().map(|item| item.clone()).collect()
    }

    pub fn get_subscriptions(&self) -> Vec<Subscription> {
        self.subscriptions.iter().map(|item| item.clone()).collect()
    }

    pub async fn create_target(&self, target: Target) -> CallResult {
        self.call(ManagementMessage::CreateTarget(target.clone()))
            .await?;
        self.targets.insert(target.id, target);
        Ok(())
    }

    pub async fn delete_target(&self, target_id: usize) -> CallResult {
        self.call(ManagementMessage::DeleteTarget(target_id))
            .await?;
        self.targets.remove(&target_id);
        Ok(())
    }

    pub async fn create_subscription(&self, subscription: Subscription) -> CallResult {
        self.call(ManagementMessage::CreateSubscription(subscription.clone()))
            .await?;
        self.subscriptions.insert(subscription.id, subscription);
        Ok(())
    }

    pub async fn delete_subscription(&self, subscription_id: usize) -> CallResult {
        self.call(ManagementMessage::DeleteSubscription(subscription_id))
            .await?;
        self.subscriptions.remove(&subscription_id);
        Ok(())
    }
}
