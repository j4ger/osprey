use dioxus::prelude::*;

use crate::components::{Loading, SubscriptionStat};

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "stats max-sm:stats-vertical",
            Loading {
                SubscriptionStat {}
            }
        }
    }
}
