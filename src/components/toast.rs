use std::time::Duration;

use async_std::task::sleep;
use dioxus::prelude::*;

const TOAST_DURATION: u64 = 5000;
const TRANSITION_DURATION: u64 = 800;

#[derive(PartialEq, Clone, Props)]
pub struct ToastProps {
    pub id: usize,
    pub message: String,
    pub kind: ToastKind,
    pub fade_out: bool,
}

#[derive(PartialEq, Clone)]
pub enum ToastKind {
    Info,
    Success,
    Warning,
    Error,
}

#[component]
pub fn SingleToast(toast: ToastProps) -> Element {
    let mut context = use_context::<Signal<ToastContext>>();

    let kind = match toast.kind {
        ToastKind::Info => "info",
        ToastKind::Success => "success",
        ToastKind::Warning => "warning",
        ToastKind::Error => "error",
    };

    let icon = match toast.kind {
        ToastKind::Info => "info-circle",
        ToastKind::Success => "square-check",
        ToastKind::Warning => "alert-hexagon",
        ToastKind::Error => "circle-x",
    };

    let title = match toast.kind {
        ToastKind::Info => "Info alert:",
        ToastKind::Success => "Success alert:",
        ToastKind::Warning => "Warning alert:",
        ToastKind::Error => "Error alert:",
    };

    rsx! {
        div { key: toast.id,
            role: "alert",
            class: "alert alert-{kind} flex items-center gap-4 transition-all duration-800 ease-in-out hover:scale-102",
            class: if toast.fade_out { "opacity-0 translate-y-10" } else { "opacity-100 translate-y-0" },
            span { class: "icon-[tabler--{icon}] size-6" }
            div { class: "flex flex-col gap-1",
                span { class: "text-lg font-semibold", "{title}" }
                p {
                    "{toast.message}"
                }
            }
            button { "aria-label": "Close Button",
                class: "ms-auto leading-none",
                onclick: move |_| { let _ = context.write().set_fade_out(toast.id); },
                span { class: "icon-[tabler--x] size-5" }
            }
        }
    }
}

// make tailwind detect these evaluated classes
// class: "alert-info alert-success alert-warning alert-error"
// class: "icon-[tabler--info-circle] icon-[tabler--square-check] icon-[tabler--alert-hexagon] icon-[tabler--circle-x]"

pub struct ToastContext {
    toasts: Vec<ToastProps>,
    next_id: usize,
}

impl ToastContext {
    pub fn new() -> Self {
        Self {
            toasts: vec![],
            next_id: 0,
        }
    }

    pub fn add_toast(&mut self, message: impl AsRef<str>, kind: ToastKind) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.toasts.push(ToastProps {
            id,
            message: message.as_ref().to_string(),
            kind,
            fade_out: false,
        });

        id
    }

    pub fn remove_toast(&mut self, id: usize) {
        self.toasts.retain(|toast| toast.id != id);
    }

    pub fn set_fade_out(&mut self, id: usize) -> bool {
        if let Some(index) = self.toasts.iter().position(|toast| toast.id == id) {
            self.toasts[index].fade_out = true;
            return true;
        } else {
            return false;
        }
    }
}

#[component]
pub fn Toasts() -> Element {
    let context = use_context::<Signal<ToastContext>>();

    rsx! {
        div { class: "fixed bottom-0 flex flex-col items-center gap-4 p-4 z-50",
            for toast in context.read().toasts.iter().rev() {
                SingleToast { toast: toast.clone() }
            }
        }
    }
}

pub async fn send_toast(message: impl AsRef<str>, kind: ToastKind) {
    let mut context = use_context::<Signal<ToastContext>>();
    let id = context.write().add_toast(message.as_ref(), kind);
    sleep(Duration::from_millis(TOAST_DURATION)).await;
    if context.write().set_fade_out(id) {
        sleep(Duration::from_millis(TRANSITION_DURATION)).await;
        context.write().remove_toast(id);
    }
}
