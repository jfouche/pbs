use dioxus::prelude::*;
use pbs_srv::Item;

pub fn PageNewItem(cx: Scope) -> Element {
    cx.render(rsx! {
        NewItem {}
        ImportCots {}
    })
}

fn NewItem(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            fieldset {
                legend { "Create new part number" }
                label { r#for: "name", "Name" }
                input {
                    oninput: |e| {}
                },
                br {},
                button { r#type: "button", "Create" }
            },
            p { "MESSAGE" }
        }
    })
}

fn ImportCots(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            fieldset {
                legend { "Import COTS" }
                label { r#for: "pn", "Part Number" }
                input {
                    oninput: |e| {}
                },
                br {},
                label { r#for: "name", "Name" }
                input {
                    oninput: |e| {}
                },
                br {},
                button { r#type: "button", "Create" }
            },
            p { "MESSAGE" }
        }
    })
}
