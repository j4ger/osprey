use dioxus::prelude::*;

use crate::components::{Loading, SubscriptionStat};

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "flex flex-col w-full items-center mt-2",
            div { class: "stats max-sm:stats-vertical",
                Loading {
                    SubscriptionStat {}
                }
            }
        }
    }
}
