use std::net::SocketAddr;

use dioxus::prelude::*;

use crate::bridge::config::Config;

use super::{subscription::load_subscriptions, target::load_targets};

pub fn launch(app: fn() -> Element) {
    let config = Config::load().unwrap();
    let targets = load_targets().unwrap();
    let subscriptions = load_subscriptions().unwrap();

    dioxus::logger::initialize_default();

    // read envvar "OSPREY_DEV", if =1 get addr and port from dioxus_cli_config::fullstack_address_or_localhost();
    // else construct from config
    let socket_addr = if dioxus_cli_config::is_cli_enabled() {
        dioxus_cli_config::fullstack_address_or_localhost()
    } else {
        SocketAddr::new(config.server.address.parse().unwrap(), config.server.port)
    };

    let router = axum::Router::new()
        .serve_dioxus_application(ServeConfig::new().unwrap(), app)
        .into_make_service();

    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async {
            let listener = tokio::net::TcpListener::bind(&socket_addr).await.unwrap();
            axum::serve(listener, router).await
        })
        .unwrap()

    // TODO: graceful shutdown?
}
