use dioxus::prelude::*;

use crate::bridge::context::get_subscription_count;
use crate::Route;

// ugly since there doesn't seem to be a way to pass in an async fn as a prop

// make a macro that spits out a component as follows, with all the placeholders evaluated
// #[component]
// pub fn {Name}Stat() -> Element {
//     let nav = navigator();
//     let count = use_server_future({future})?;

//     rsx! {
//         div { class: "stat btn h-full",
//             onclick: move |_| {nav.push({route});},
//             div { class: "stat-figure text-base-content size-8",
//                 span { class: "icon-[tabler--{icon}] size-8" }
//             }
//             div { class: "stat-title", {title} }
//             div { class: "stat-value", "{count}" }
//         }
//     }
// }

macro_rules! gen_stat {
    ($name:ident, $future:expr, $route:expr, $icon:literal, $title:expr) => {
        #[component]
        pub fn $name() -> Element {
            let nav = navigator();
            let count = use_server_future($future)?.suspend()?;

            rsx! {
                div { class: "stat btn h-full",
                    onclick: move |_| {nav.push($route);},
                    div { class: "stat-figure text-base-content size-8",
                        span { class: $icon, class: "size-8" }
                    }
                    div { class: "stat-title", $title }
                    match count.read_unchecked().as_ref() {
                        Ok(value) => rsx! { div { class: "stat-value", "{value}" } },
                        Err(_) => rsx! { div { class: "stat-value", "Err!" } }
                    }
                }
            }
        }
    };
}

gen_stat!(
    SubscriptionStat,
    get_subscription_count,
    Route::Subscriptions {},
    "icon-[tabler--rss]",
    "Subscriptions"
);
