use dioxus::{prelude::*, Result};

use crate::bridge::{context::get_subscription_count, subscription::Subscription};

use crate::utils::time::time_ago;

// TODO: Pagination

#[component]
pub fn Subscriptions() -> Element {
    let count_future = use_server_future(get_subscription_count)?;
    let count = count_future.suspend()?;

    let current_subscription: Signal<Option<Subscription>> = use_signal(|| None);

    let subscriptions: Signal<Result<Vec<Subscription>>> = use_signal(|| {
        Ok(vec![
            Subscription {
                id: 0,
                interval: 1000,
                last_update: 110294990,
                name: "test name".into(),
                url: "test url".into(),
                push_targets: vec![0, 1],
                update_count: 114,
            },
            {
                Subscription {
                    id: 1,
                    interval: 1000,
                    last_update: 91192489,
                    name: "test".into(),
                    url: "test".into(),
                    push_targets: vec![2],
                    update_count: 514,
                }
            },
        ])
    });

    rsx! {
        div {
            role: "dialog",
            tabindex: "-1",
            class: "overlay modal overlay-open:opacity-100 modal-middle hidden",
            id: "confirm-modal",
            div { class: "modal-dialog overlay-open:opacity-100",
                div { class: "modal-content",
                    div { class: "modal-header",
                        h3 { class: "modal-title", "Confirm Delete?" }
                        button {
                            r#type: "button",
                            "aria-label": "Close",
                            "data-overlay": "#middle-center-modal",
                            class: "btn btn-text btn-circle btn-sm absolute end-3 top-3",
                            span { class: "icon-[tabler--x] size-4" }
                        }
                    }
                    div { class: "modal-body",
                        "Are you sure you want to delete this subscription? This action cannot be undone."
                    }
                    div { class: "modal-footer",
                        button {
                            r#type: "button",
                            "data-overlay": "#confirm-modal",
                            class: "btn btn-warning",
                            "Confirm"
                        }
                        button {
                            r#type: "button",
                            "data-overlay": "#confirm-modal",
                            class: "btn btn-primary",
                            "Cancel"
                        }
                    }
                }
            }
        }

        div {
            div { class: "px-4 sm:px-0",
                h3 { class: "text-2xl font-semibold text-base-content/90", "Subscriptions" }
                p { class: "mt-1 max-w-full text-base-content/80",
                    match count() {
                        Ok(count) => {
                            format!("{count} subscriptions active.")
                        }
                        Err(_) => "Failed to load count...".into()
                    }
                }
            }
            div { class: "mt-6 border-t border-base-content/25",
                div { class: "list-inside list-none",
                    match subscriptions() {
                        Ok(subscriptions) => {
                            rsx! {
                                for subscription in subscriptions.iter() {
                                    div { class: "card sm:card-side max-w-sm sm:max-w-full mt-2",
                                        div { class: "card-body",
                                            h5 { class: "card-title mb-2.5", "{subscription.name}" }
                                            p { class: "mb-3",
                                                "{subscription.url}"
                                            }
                                            div { class: "card-actions",
                                                button { class: "btn btn-accent",
                                                    role: "button",
                                                    "Edit"
                                                }
                                                button { class: "btn btn-warning",
                                                    role: "button",
                                                    "aria-haspopup": "dialog",
                                                    "aria-expanded": "false",
                                                    "aria-controls": "confirm-modal",
                                                    "data-overlay":"#confirm-modal",
                                                    "Delete"
                                                }
                                            }
                                        }
                                        div { class: "card-side p-2 hidden md:block",
                                            div { class: "stats border-base-content/10 border shadow-none",
                                                div { class: "stat",
                                                    div { class: "stat-figure text-base-content size-8",
                                                        span { class: "icon-[tabler--clock] size-8" }
                                                    }
                                                    div { class: "stat-title", "Last Updated" }
                                                    div { class: "stat-value", "{time_ago(subscription.last_update)}" }
                                                }
                                                div { class: "stat",
                                                    div { class: "stat-figure text-base-content size-8",
                                                        span { class: "icon-[tabler--chart-dots] size-8" }
                                                    }
                                                    div { class: "stat-title", "Update Count" }
                                                    div { class: "stat-value", "{subscription.update_count}" }
                                                }
                                                div { class: "stat",
                                                    div { class: "stat-figure text-base-content size-8",
                                                        span { class: "icon-[tabler--bell] size-8" }
                                                    }
                                                    div { class: "stat-title", "Push Targets" }
                                                    div { class: "stat-value", "{subscription.push_targets.len()}" }
                                                }
                                            }
                                        }
                                    }
                               }
                            }
                        }
                        Err(_) => {
                            rsx! { "Failed to fetch subscriptions." }
                        }
                    }
                }
            }
        }
    }
}
