use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        h1 {"test"}
        button { class: "btn border-[#2b3137] bg-[#2b3137] text-white shadow-[#2b3137]/30 hover:border-[#2b3137] hover:bg-[#2b3137]/90",
            span { class: "icon-[tabler--brand-github]" }
            " Github"
        }
    }
}
