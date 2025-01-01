use dioxus::prelude::*;

#[component]
pub fn Spinner() -> Element {
    rsx! {
        span {
            class: "loading loading-spinner"
        }
    }
}

#[component]
pub fn SpinnerLg() -> Element {
    rsx! {
        span {
            class: "loading loading-spinner loading-lg"
        }
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct LoadingProps {
    children: Element,
}

#[component]
pub fn Loading(props: LoadingProps) -> Element {
    rsx! {
        SuspenseBoundary {
            fallback: |context: SuspenseContext|
            rsx! {
                if let Some(placeholder) = context.suspense_placeholder() {
                    {placeholder}
                } else {
                    div { class: "p-4",
                        SpinnerLg {}
                    }
                }
            },
            { props.children }
        }
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct GlobalLoadingContext {
    loading: bool,
}

impl GlobalLoadingContext {
    pub fn new() -> Self {
        Self { loading: false }
    }

    pub fn set_loading(&mut self, loading: bool) {
        self.loading = loading;
    }
}

#[component]
pub fn GlobalLoading() -> Element {
    let context = use_context::<Signal<GlobalLoadingContext>>();

    rsx! {
        div { class: "overlay-backdrop transition duration fixed inset-0 bg-base-shadow/70 overflow-y-auto z-50",
            class: if context().loading { "open" } else { "hidden" },
            div { class: "grid place-items-center h-full",
                SpinnerLg {}
            }
        }
    }
}

pub fn set_loading() {
    let mut context = use_context::<Signal<GlobalLoadingContext>>();
    context.write().set_loading(true);
}

pub fn unset_loading() {
    let mut context = use_context::<Signal<GlobalLoadingContext>>();
    context.write().set_loading(false);
}
