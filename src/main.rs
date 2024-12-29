use dioxus::prelude::*;

use components::Navbar;
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

#[allow(non_snake_case)]
#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }

        div { class: "min-h-screen bg-gradient-to-tr from-base-200 to-base-100",
            Router::<Route> {}
        }
    }
}

// TODO: logging
// TODO: error handling using ErrorBoundary
