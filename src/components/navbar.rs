use dioxus::prelude::*;

use crate::bridge::context::{get_run_state, set_run_state};
use crate::components::{Loading, Spinner};
use crate::Route;

const LOGO: Asset = asset!("/assets/osprey.png");

#[component]
pub fn Navbar() -> Element {
    let mut get_state_future = use_server_future(get_run_state)?;
    let run_state_response = get_state_future.suspend()?;
    let run_state = use_signal(|| run_state_response().unwrap_or(false));

    let mut flipping = use_signal(|| false);

    rsx! {
        nav { class: "navbar sticky top-0 z-10 rounded-box shadow-md justify-between backdrop-blur",
            div { class: "navbar-start",
                a { class: "avatar", href: "/",
                    div { class: "size-12 rounded-full",
                        img { src: LOGO, alt: "logo" }
                    }
                }

            }
            div { class: "navbar-center flex items-center",
                a {
                    href: "/",
                    class: "link text-base-content/90 link-neutral text-xl font-semibold no-underline",
                    "Osprey"
                }
            }
            div { class: "navbar-end",
                div { class: "space-x-3",
                    class: if flipping() { "cursor-progress" } else { "cursor-pointer" },
                    onclick: move |_| async move {
                        if flipping() { return; }
                        flipping.set(true);
                        let _ = set_run_state(!run_state()).await; // TODO: this map produce error in the future
                        get_state_future.restart();
                        flipping.set(false);
                    },
                   label { class: "relative inline-block",
                        div { class: "absolute start-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 transform pt-2",
                            class: if !flipping() { "hidden" },
                            Spinner {}
                        }
                        input {
                            "aria-label": "default switch with icon",
                            r#type: "checkbox",
                            checked: run_state(), // somehow this is inverted
                            disabled: flipping(),
                            class: "switch switch-primary peer mt-2",
                        }
                        span { class: "icon-[tabler--check] peer-checked:text-primary-content absolute start-1 hidden size-4 peer-checked:block top-3" }
                        span { class: "icon-[tabler--zzz] text-base-content peer-checked:text-base-content/90 absolute end-1.5 block size-4 peer-checked:hidden top-3" }
                    }
                }
            }
        }

        div { class: "flex flex-col w-full items-center pt-4 px-8 z-0",
            div { class: "container w-fit",
                Loading {
                    Outlet::<Route> {}
                }
            }
        }
    }
}
