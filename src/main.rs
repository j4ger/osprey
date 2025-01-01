use dioxus::prelude::*;

use components::{GlobalLoading, GlobalLoadingContext, Navbar, ToastContext, Toasts};
use views::*;

mod bridge;
mod components;
mod utils;
mod views;

#[cfg(feature = "server")]
mod server;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},

    #[route("/subscriptions")]
    Subscriptions {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");

fn main() {
    #[cfg(not(feature = "server"))]
    dioxus::launch(App);
    #[cfg(feature = "server")]
    crate::server::launch(App);
}

#[component]
fn App() -> Element {
    let toast_ctx = ToastContext::new();
    provide_context(use_signal(move || toast_ctx));

    let loading_ctx = GlobalLoadingContext::new();
    provide_context(use_signal(move || loading_ctx));

    rsx! {
        document::Link { rel: "icon", href: FAVICON }

        div { class: "min-h-screen bg-gradient-to-tr from-base-200 to-base-100",
            Router::<Route> {}
        }

        Toasts {}
        GlobalLoading {}
    }
}

// TODO: logging
// TODO: error handling using ErrorBoundary
