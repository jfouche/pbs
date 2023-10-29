use dioxus::prelude::*;

#[derive(Props, PartialEq)]
pub struct ItemIdProps {
    pub id: i64,
}

pub fn page_view_item(cx: Scope<ItemIdProps>) -> Element {
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
