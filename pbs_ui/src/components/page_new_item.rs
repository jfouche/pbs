use crate::client;
use dioxus::prelude::*;
use futures_util::StreamExt;

pub fn page_new_item(cx: Scope) -> Element {
    cx.render(rsx! {
        new_item {}
        import_cots {}
    })
}

fn new_item(cx: Scope) -> Element {
    let name = use_state(cx, || "".to_string());
    let message = use_state(cx, || "".to_string());

    let new_item_handler = use_coroutine(cx, |mut rx: UnboundedReceiver<String>| {
        to_owned![message];
        async move {
            while let Some(name) = rx.next().await {
                let result = client::new_item(&name).await;
                let msg = match result {
                    Ok(item) => format!("Item [{}] created", item.pn()),
                    Err(e) => format!("ERROR : {e:?}"),
                };
                message.set(format!("[[{name}]] : {msg}"));
            }
        }
    });

    cx.render(rsx! {
        div {
            fieldset {
                legend { "Create new part number" }
                label { r#for: "name", "Name" }
                input {
                    name: "name",
                    value: "{name}",
                    onmounted: move |evt| {evt.data.set_focus(true);},
                    oninput: move |evt| name.set(evt.value.clone()),
                },
                br {},
                button {
                    onclick: move |_| {
                        new_item_handler.send(name.get().to_owned())
                    }
                    , "Create"
                }
            },
            div { "{message}"}
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
