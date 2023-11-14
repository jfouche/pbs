use dioxus::prelude::*;

pub fn panel_recently_use(cx: Scope) -> Element {
    render! {
        div {
            style:"flex: 0 120px",
            p { "Recently use" }}
    }
}
