use dioxus::prelude::*;

use super::{subscription::Subscription, target::Target};

#[cfg(feature = "server")]
use crate::server::AppContext;

type Result<T> = std::result::Result<T, ServerFnError>;

#[server]
pub async fn get_targets() -> Result<Vec<Target>> {
    Ok(AppContext::get().await.get_targets())
}

#[server]
pub async fn get_target_count() -> Result<usize> {
    Ok(AppContext::get().await.get_target_count())
}

#[server]
pub async fn create_target(target: Target) -> Result<()> {
    AppContext::get()
        .await
        .create_target(target)
        .await
        .map_err(|err| ServerFnError::ServerError(err.to_string()))
}

#[server]
pub async fn delete_target(target_id: usize) -> Result<()> {
    AppContext::get()
        .await
        .delete_target(target_id)
        .await
        .map_err(|err| ServerFnError::ServerError(err.to_string()))
}

#[server]
pub async fn get_subscriptions() -> Result<Vec<Subscription>> {
    Ok(AppContext::get().await.get_subscriptions())
}

#[server]
pub async fn get_subscription_count() -> Result<usize> {
    Ok(AppContext::get().await.get_subscription_count())
}

#[server]
pub async fn create_subscription(subscription: Subscription) -> Result<()> {
    AppContext::get()
        .await
        .create_subscription(subscription)
        .await
        .map_err(|err| ServerFnError::ServerError(err.to_string()))
}

#[server]
pub async fn delete_subscription(subscription_id: usize) -> Result<()> {
    AppContext::get()
        .await
        .delete_subscription(subscription_id)
        .await
        .map_err(|err| ServerFnError::ServerError(err.to_string()))
}
