use crate::Route;
use dioxus::prelude::*;

const LOGO: Asset = asset!("/assets/osprey.png");

#[component]
pub fn Navbar() -> Element {
    rsx! {
        nav { class: "navbar rounded-box shadow",
            div { class: "w-full md:flex md:items-center md:gap-2",
                div { class: "flex items-center justify-between",
                    div { class: "navbar-start items-center md:gap-2 justify-between max-md:w-full",
                        a { class: "avatar",
                            href: "/",
                            div { class: "size-12 rounded-full",
                                img {
                                    src: LOGO,
                                    alt: "logo",
                                }
                            }
                        }
                        a {
                            href: "/",
                            class: "link text-base-content/90 link-neutral text-xl font-semibold no-underline",
                            "Osprey"
                        }
                        div { class: "md:hidden",
                            button {
                                r#type: "button",
                                "data-collapse": "#default-navbar-collapse",
                                "aria-label": "Toggle navigation",
                                "aria-controls": "default-navbar-collapse",
                                class: "collapse-toggle btn btn-outline btn-secondary btn-sm btn-square",
                                span { class: "icon-[tabler--menu-2] collapse-open:hidden size-4" }
                                span { class: "icon-[tabler--x] collapse-open:block hidden size-4" }
                            }
                        }
                    }
                }
                div {
                    class: "md:navbar-end collapse hidden grow basis-full overflow-hidden transition-[height] duration-300 max-md:w-full",
                    id: "default-navbar-collapse",
                    ul { class: "menu md:menu-horizontal gap-2 p-0 text-base max-md:mt-2",
                        li {
                            Link {
                                to: Route::Home {},
                                "Home"
                            }
                        }
                        li {
                            a { class: "btn border-[#2b3137] bg-[#2b3137] text-white shadow-[#2b3137]/30 hover:border-[#2b3137] hover:bg-[#2b3137]/90",
                                href: "https://github.com/j4ger/osprey",
                                target: "_blank",
                                span { class: "icon-[tabler--brand-github]" }
                                " Github"
                            }
                        }
                    }
                }
            }
        }

        Outlet::<Route> {}
    }
}
