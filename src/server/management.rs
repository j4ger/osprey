use std::collections::HashMap;

use anyhow::bail;
use tokio::{
    select,
    sync::{broadcast, mpsc, oneshot},
};

use crate::bridge::{config::Config, subscription::Subscription, target::Target};

use super::{
    message::{StatMessage, UpdateMessage},
    subscription::{handle_subscription, save_subscriptions},
    target::{handle_target, save_targets},
    task::Task,
};

pub type CallResult = Result<(), ManagementError>;

pub struct ManagementMessageCapsule {
    pub message: ManagementMessage,
    pub response: oneshot::Sender<CallResult>,
}

// TODO: health check?
pub enum ManagementMessage {
    CreateTarget(Target),
    DeleteTarget(usize),
    ModifyTarget(Target),
    CreateSubscription(Subscription),
    DeleteSubscription(usize),
    ModifySubscription(Subscription),
    UpdateConfig(Config),
}

pub type ManagementError = anyhow::Error;

pub async fn handle_management(
    mut rx: mpsc::Receiver<ManagementMessageCapsule>,
    mut terminator: oneshot::Receiver<()>,
    config: Config,
    targets: Vec<Target>,
    subscriptions: Vec<Subscription>,
) {
    let mut state = ManagementState::new(targets, subscriptions, config);

    loop {
        select! {
            _ = &mut terminator => {
                state.shutdown();
                break;
            }
            message = rx.recv() => {
                if let Some(message) = message {
                    let result = match message.message {
                        ManagementMessage::CreateTarget(target) => {
                            state.create_target(target)
                        }
                        ManagementMessage::DeleteTarget(target_id) => {
                            state.delete_target(target_id)
                        }
                        ManagementMessage::CreateSubscription(subscription) => {
                            state.create_subscription(subscription)
                        }
                        ManagementMessage::ModifyTarget(target) => {
                            state.modify_target(target)
                        }
                        ManagementMessage::DeleteSubscription(subscription_id) => {
                            state.delete_subscription(subscription_id)
                        }
                        ManagementMessage::ModifySubscription(subscription) => {
                            state.modify_subscription(subscription)
                        }
                        ManagementMessage::UpdateConfig(config) => {
                            config.save().unwrap();
                            state.config = config;
                            Ok(())
                        }
                    };
                    message.response.send(result).unwrap();
                } else {
                    // log
                    state.shutdown();
                    break;
                }
            }
        }
    }
}

// TODO: in the end, we still need a manager task
// but we can share less AppState and I don't think it would need a Vec<Target> and Vec<Subscription>
// also, as all targets and subscriptions are id-ed, we can use DashMaps instead
// the manager would need to handle the creation and deletion of targets and subscriptions
// modification of targets and subscriptions can be implemented in the corresponding server fn, with two calls
struct ManagementState {
    config: Config,
    broadcast_tx: broadcast::Sender<UpdateMessage>,
    log_tx: mpsc::Sender<StatMessage>,
    targets: Vec<Target>,
    target_tasks: HashMap<usize, Task>,
    subscriptions: Vec<Subscription>,
    subscription_tasks: HashMap<usize, Task>,
    log_task: Task,
}

impl ManagementState {
    pub fn new(targets: Vec<Target>, subscriptions: Vec<Subscription>, config: Config) -> Self {
        let (broadcast_tx, _) = broadcast::channel(config.server.queue_size);

        let (log_terminator, log_terminator_rx) = oneshot::channel();
        let (log_tx, log_rx) = mpsc::channel(config.server.logger_queue_size);

        let log_handle = tokio::spawn(super::logger::handle_log(log_rx, log_terminator_rx));
        let log_task = Task::new(log_terminator, log_handle);

        let mut target_tasks = HashMap::new();
        for target in targets.iter() {
            let (terminator_tx, terminator_rx) = oneshot::channel();
            let handle = tokio::spawn(handle_target(
                target.clone(),
                broadcast_tx.subscribe(),
                terminator_rx,
                log_tx.clone(),
            ));
            target_tasks.insert(target.id, Task::new(terminator_tx, handle));
        }

        let mut subscription_tasks = HashMap::new();
        for subscription in subscriptions.iter() {
            let (terminator_tx, terminator_rx) = oneshot::channel();
            let handle = tokio::spawn(handle_subscription(
                subscription.clone(),
                broadcast_tx.clone(),
                terminator_rx,
                log_tx.clone(),
            ));
            subscription_tasks.insert(subscription.id, Task::new(terminator_tx, handle));
        }

        Self {
            config,
            broadcast_tx,
            log_tx,
            targets,
            target_tasks,
            subscriptions,
            subscription_tasks,
            log_task,
        }
    }

    pub fn shutdown(self) {
        for (_, task) in self.target_tasks {
            task.kill();
        }

        for (_, task) in self.subscription_tasks {
            task.kill();
        }

        self.log_task.kill();
    }

    pub fn create_target(&mut self, target: Target) -> CallResult {
        if self.targets.iter().any(|t| t.id == target.id) {
            bail!("Target ID already exists")
        }
        let (terminator_tx, terminator_rx) = oneshot::channel();
        let handle = tokio::spawn(handle_target(
            target.clone(),
            self.broadcast_tx.subscribe(),
            terminator_rx,
            self.log_tx.clone(),
        ));
        self.target_tasks
            .insert(target.id, Task::new(terminator_tx, handle));
        self.targets.push(target);
        save_targets(&self.targets).unwrap();
        Ok(())
    }

    pub fn delete_target(&mut self, target_id: usize) -> CallResult {
        if let Some(task) = self.target_tasks.remove(&target_id) {
            task.kill();
            self.targets.retain(|target| target.id != target_id);
            save_targets(&self.targets).unwrap();
            Ok(())
        } else {
            bail!("Target not found.")
        }
    }

    pub fn modify_target(&mut self, target: Target) -> CallResult {
        if let Some(task) = self.target_tasks.remove(&target.id) {
            task.kill();
            self.targets.retain(|t| t.id != target.id);
            self.create_target(target);
            save_targets(&self.targets).unwrap();
            Ok(())
        } else {
            bail!("Target not found.")
        }
    }

    pub fn create_subscription(&mut self, subscription: Subscription) -> CallResult {
        if self.subscriptions.iter().any(|s| s.id == subscription.id) {
            bail!("Subscription ID already exists.")
        }
        let (terminator_tx, terminator_rx) = oneshot::channel();
        let handle = tokio::spawn(handle_subscription(
            subscription.clone(),
            self.broadcast_tx.clone(),
            terminator_rx,
            self.log_tx.clone(),
        ));
        self.subscription_tasks
            .insert(subscription.id, Task::new(terminator_tx, handle));
        self.subscriptions.push(subscription);
        save_subscriptions(&self.subscriptions).unwrap();
        Ok(())
    }

    pub fn delete_subscription(&mut self, subscription_id: usize) -> CallResult {
        if let Some(task) = self.subscription_tasks.remove(&subscription_id) {
            task.kill();
            self.subscriptions
                .retain(|subscription| subscription.id != subscription_id);
            save_subscriptions(&self.subscriptions).unwrap();
            Ok(())
        } else {
            bail!("Subscription not found.")
        }
    }

    pub fn modify_subscription(&mut self, subscription: Subscription) -> CallResult {
        if let Some(task) = self.subscription_tasks.remove(&subscription.id) {
            task.kill();
            self.subscriptions.retain(|s| s.id != subscription.id);
            self.create_subscription(subscription);
            save_subscriptions(&self.subscriptions).unwrap();
            Ok(())
        } else {
            bail!("Subscription not found.")
        }
    }
}
