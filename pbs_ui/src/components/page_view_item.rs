use dioxus::prelude::*;

pub fn page_view_item(cx: Scope) -> Element {
    render! {
        div {
            class: "tree",
            h2 {"PN / NAME"}
            // TreeItem {}
            // AddChild {}
            "Search item"
        }
    }
}
