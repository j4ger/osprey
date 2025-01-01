use dioxus::prelude::*;

use crate::components::Loading;
use crate::Route;

const LOGO: Asset = asset!("/assets/osprey.png");

#[component]
pub fn Navbar() -> Element {
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
                    label { class: "relative inline-block",
                        input {
                            "aria-label": "default switch with icon",
                            r#type: "checkbox",
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
