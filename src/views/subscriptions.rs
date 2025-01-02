use dioxus::{prelude::*, Result};

use crate::bridge::context::delete_subscription;
use crate::bridge::{context::get_subscription_count, subscription::Subscription};

use crate::components::{send_toast, set_loading, unset_loading, Modal, ToastKind};

use crate::utils::time::time_ago;

// TODO: Pagination

#[component]
pub fn Subscriptions() -> Element {
    let mut confirm_open = use_signal(|| false);

    let mut edit_open = use_signal(|| false);

    let count_future = use_server_future(get_subscription_count)?;
    let count = count_future.suspend()?;

    let mut current_subscription: Signal<Subscription> = use_signal(|| Subscription::empty());
    let mut new_subscription = use_signal(|| false);

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
        Modal {
            open: confirm_open,
            div { class: "modal-content",
                div { class: "modal-header",
                    h3 { class: "modal-title", "Confirm Delete?" }
                    button {
                        r#type: "button",
                        "aria-label": "Close",
                        class: "btn btn-text btn-circle btn-sm absolute end-3 top-3",
                        onclick: move |_| {
                            confirm_open.set(false);
                        },
                        span { class: "icon-[tabler--x] size-4" }
                    }
                }
                div { class: "modal-body",
                    "Are you sure you want to delete this subscription: {current_subscription().name}? This action cannot be undone."
                }
                div { class: "modal-footer",
                    button {
                        r#type: "button",
                        class: "btn btn-warning",
                        onclick: move |_| async move {
                            set_loading();
                            if new_subscription() {
                                unset_loading();
                                return;
                            }

                            match delete_subscription(current_subscription().id).await {
                                Ok(_) => {
                                    unset_loading();
                                    confirm_open.set(false);

                                    send_toast("Removed.", ToastKind::Success).await;
                                }
                                Err(err) => {
                                    unset_loading();
                                    confirm_open.set(false);

                                    send_toast(format!("{}", err), ToastKind::Error).await;
                                }
                            }

                        },
                        "Confirm"
                    }
                    button {
                        r#type: "button",
                        class: "btn btn-primary",
                        onclick: move |_| confirm_open.set(false),
                        "Cancel"
                    }
                }
            }
        }

        Modal {
            open: edit_open,
            scrollable: true,
            div { class: "modal-content",
                div { class: "modal-header",
                    h3 { class: "modal-title",
                        if new_subscription() {
                            "New Subscription"
                        } else {
                            "Edit Subscription: {current_subscription().name}"
                        }
                    }
                    button {
                        r#type: "button",
                        "aria-label": "Close",
                        class: "btn btn-text btn-circle btn-sm absolute end-3 top-3",
                        onclick: move |_| edit_open.set(false),
                        span { class: "icon-[tabler--x] size-4" }
                    }
                }
                form {
                    div { class: "modal-body pt-0",
                        div { class: "mb-4",
                            label { r#for: "name", class: "label label-text", " Name " }
                            input {
                                r#type: "text",
                                placeholder: "",
                                class: "input",
                                id: "name",
                                value: current_subscription().name,
                                oninput: move |event| current_subscription.with_mut(|s| {
                                    s.name = event.value();
                                })
                            }
                        }
                        div { class: "mb-4",
                            label { r#for: "url", class: "label label-text", " URL " }
                            input {
                                r#type: "text",
                                placeholder: "RSS Feed URL",
                                class: "input",
                                id: "url",
                                value: current_subscription().url,
                                oninput: move |event| current_subscription.with_mut(|s| {
                                    s.url = event.value();
                                })
                            }
                        }

                        div { class: "mb-0.5 flex gap-4 max-sm:flex-col",
                            div { class: "w-full",
                                label { r#for: "email", class: "label label-text", " Email " }
                                input {
                                    placeholder: "johndoe@123@gmail.com",
                                    r#type: "email",
                                    class: "input",
                                    id: "email",
                                }
                            }
                            div { class: "w-full",
                                label { r#for: "dateOfBirth", class: "label label-text", " DOB " }
                                input {
                                    r#type: "date",
                                    class: "input",
                                    id: "dateOfBirth",
                                }
                            }
                        }
                    }
                    div { class: "modal-footer",
                        button {
                            r#type: "button",
                            class: "btn btn-soft btn-secondary",
                            onclick: move |_| edit_open.set(false),
                            "Close"
                        }
                        button { r#type: "submit",
                            class: "btn btn-primary",
                            onclick: move |_| {

                            },
                            "Save changes"
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
                                for subscription in subscriptions {
                                    div { class: "card sm:card-side max-w-sm sm:max-w-full mt-2",
                                        div { class: "card-body",
                                            h5 { class: "card-title mb-2.5", "{subscription.name}" }
                                            p { class: "mb-3",
                                                "{subscription.url}"
                                            }
                                            div { class: "card-actions",
                                                {
                                                    let subscription = subscription.clone();
                                                    let subscription_clone = subscription.clone();
                                                    rsx! {
                                                        button { class: "btn btn-accent",
                                                            role: "button",
                                                            onclick: move |_| {
                                                                current_subscription.set(subscription_clone.clone());
                                                                new_subscription.set(false);
                                                                edit_open.set(true);
                                                            },
                                                            "Edit"
                                                        }
                                                        button { class: "btn btn-warning",
                                                            role: "button",
                                                            onclick: move |_| {
                                                                current_subscription.set(subscription.clone());
                                                                new_subscription.set(false);
                                                                confirm_open.set(true);
                                                            },
                                                            "Delete"
                                                        }
                                                    }
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
