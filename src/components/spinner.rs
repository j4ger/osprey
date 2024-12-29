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
