use dioxus::prelude::*;

pub fn panel_history(cx: Scope) -> Element {
    render! {
        div {
            style:"flex: 0 120px",
            p { "HISTORY" }}
    }
}
