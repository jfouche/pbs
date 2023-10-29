use crate::client;
use dioxus::prelude::*;
use futures_util::StreamExt;

pub fn page_new_item(cx: Scope) -> Element {
    render! {
        make_item {}
        buy_item {}
    }
}

fn make_item(cx: Scope) -> Element {
    let name = use_state(cx, String::new);
    let message = use_state(cx, String::new);

    let make_item_handler = use_coroutine(cx, |mut rx: UnboundedReceiver<String>| {
        to_owned![message];
        async move {
            while let Some(name) = rx.next().await {
                let msg = match client::item_make(&name).await {
                    Ok(item) => format!("Item MAKE [{}] created", item.pn()),
                    Err(e) => format!("ERROR : {e:?}"),
                };
                message.set(msg);
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
                        make_item_handler.send(name.get().to_owned())
                    }
                    , "Create"
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

    let buy_item_handler = use_coroutine(cx, |mut rx: UnboundedReceiver<(String, String)>| {
        to_owned![message];
        async move {
            while let Some((pn, name)) = rx.next().await {
                let msg = match client::item_buy(&pn, &name).await {
                    Ok(_) => "Item BUY created".to_string(),
                    Err(e) => format!("ERROR : {e:?}"),
                };
                message.set(msg);
            }
        }
    });

    render! {
        div {
            fieldset {
                legend { "Import COTS" }
                label { r#for: "pn", "Part Number" }
                input { name: "cots_name", "value": "{name}" },
                br {},
                label { r#for: "name", "Name" }
                input { name: "cots_pn", "value": "{pn}" }
                br {},
                button { onclick: move |_| {
                    buy_item_handler.send((pn.get().to_owned(), name.get().to_owned()))
                }, "Create" }
            },
            p { "MESSAGE" }
        }
    }
}
