use dioxus::prelude::*;

use crate::client;

pub fn page_new_item(cx: Scope) -> Element {
    cx.render(rsx! {
        new_item {}
        import_cots {}
    })
}

fn new_item(cx: Scope) -> Element {
    let name = use_state(cx, || "".to_string());

    let n = name.get().clone();

    cx.render(rsx! {
        div {
            fieldset {
                legend { "Create new part number" }
                label { r#for: "name", "Name" }
                input { name: "name", "value": "{name}" },
                br {},
                button { "Create" }
                // button { onclick: move |_| async move { let _ = client::new_item(&n).await; }, "Create" }
            },
            p { "MESSAGE" }
        }
    })
}

fn import_cots(cx: Scope) -> Element {
    let name = use_state(cx, || "".to_string());
    let pn = use_state(cx, || "".to_string());
    cx.render(rsx! {
        div {
            fieldset {
                legend { "Import COTS" }
                label { r#for: "pn", "Part Number" }
                input { name: "cots_name", "value": "{name}" },
                br {},
                label { r#for: "name", "Name" }
                input { name: "cots_pn", "value": "{pn}" }
                br {},
                button { r#type: "button", "Create" }
            },
            p { "MESSAGE" }
        }
    })
}
