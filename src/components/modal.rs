use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct ModalProps {
    open: Signal<bool>,
    #[props(default = false)]
    scrollable: bool,
    children: Element,
}

#[component]
pub fn Modal(
    ModalProps {
        open,
        children,
        scrollable,
    }: ModalProps,
) -> Element {
    rsx! {
        div {
            class: "overlay-backdrop transition-all duration-300 fixed inset-0 bg-base-shadow/70 overflow-y-auto",
            class: if open() { "opacity-100 z-30" } else { "z-0 opacity-0" },
        }
        div {
            role: "dialog",
            tabindex: "-1",
            class: "overlay modal modal-middle transition-all",
            class: if scrollable { "[--body-scroll:true]" },
            class: if open() { "opacity-100 z-30" } else { "z-0" },
            div { class: "modal-dialog transition-transform transform",
                class: if open() { "opacity-100" } else { "scale-75" },
                { children }
            }
        }
    }
}
