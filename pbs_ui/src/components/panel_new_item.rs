use crate::service::{buy_item_coroutine, make_item_coroutine};
use dioxus::prelude::*;

pub fn panel_new_item(cx: Scope) -> Element {
    render! {
        make_item {}
        buy_item {}
    }
}

fn make_item(cx: Scope) -> Element {
    let name = use_state(cx, String::new);
    let message = use_state(cx, String::new);
    let make_item_handler = use_coroutine(cx, |rx| make_item_coroutine(rx, message.to_owned()));

    cx.render(rsx! {
        div {
            fieldset {
                legend { "Create new part number" }
                label { r#for: "name", "Name" }
                input {
                    class: "w3-border w3-padding",
                    name: "name",
                    value: "{name}",
                    onmounted: move |evt| { evt.data.set_focus(true); },
                    oninput: move |evt| name.set(evt.value.clone()),
                },
                br {},
                button {
                    class: "w3-button w3-theme",
                    onclick: move |_| {
                        make_item_handler.send(name.get().to_owned())
                    },
                    "Create"
                }
            },
            div { "{message}"}
        }
    })
}

fn buy_item(cx: Scope) -> Element {
    let pn = use_state(cx, String::new);
    let name = use_state(cx, String::new);
    let message = use_state(cx, String::new);
    let buy_item_handler = use_coroutine(cx, |rx| buy_item_coroutine(rx, message.to_owned()));

    render! {
        div {
            fieldset {
                legend { "Import COTS" }
                label { r#for: "pn", "Part Number" }
                input {
                    class: "w3-border w3-padding",
                    name: "cots_name",
                    value: "{name}",
                    oninput: move |evt| name.set(evt.value.clone()),
                },
                br {},
                label { r#for: "name", "Name" }
                input {
                    class: "w3-border w3-padding",
                    name: "cots_pn",
                    value: "{pn}",
                    oninput: move |evt| pn.set(evt.value.clone()),
                }
                br {},
                button {
                    class: "w3-button w3-theme",
                    onclick: move |_| {
                        buy_item_handler.send((pn.get().to_owned(), name.get().to_owned()))
                    },
                    "Create" }
            },
            p { "MESSAGE" }
        }
    }
}
