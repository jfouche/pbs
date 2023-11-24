use dioxus::prelude::*;

use pbs_srv::Item;

use crate::{
    components::commons::{item_descr, item_quantity},
    service::{delete_child_service, load_children_service, load_item_service, search_coroutine},
};

use super::ItemIdProps;

pub fn panel_edit_item(cx: Scope<ItemIdProps>) -> Element {
    let item_future = use_future(cx, (), |_| load_item_service(cx.props.id));
    let children_future = use_future(cx, (), |_| load_children_service(cx.props.id));

    render! {
        div {
            id: "edit-item",
            match item_future.value() {
                Some(item) => rsx!(
                    h2 {
                        "Item : {item.name()}"
                    }
                    item_descr { item: item.clone() },
                    match children_future.value() {
                        None => rsx! { "Loading..."} ,
                        Some(children) => rsx! {
                            ul {
                                children.into_iter().map(|child| rsx! {
                                    li {
                                        item_descr { item: child.item.clone() }
                                        item_quantity { quantity: child.quantity }
                                    }
                                    button {
                                        class: "w3-button w3-theme",
                                        onclick: move |_| {
                                            delete_child_service(cx, item.id(), child.id());
                                            children_future.restart();
                                        },
                                        "Delete"
                                    }
                                })
                             }
                        }
                    }
                    hr {}
                    panel_update { }
                ),
                None => rsx!(p { "loading" })
            }
        }
    }
}

fn panel_update(cx: Scope) -> Element {
    let results = use_state(cx, Vec::<Item>::new);
    let message = use_state(cx, String::new);
    let search_handler = use_coroutine(cx, |rx| {
        search_coroutine(rx, results.to_owned(), message.to_owned())
    });

    render! {
        div {
            input {
                class: "w3-border w3-padding",
                list: "results",
                oninput: move |evt| {
                    let pattern = evt.value.to_owned();
                    if pattern.len() > 2 {
                        search_handler.send(pattern);
                    }
                }
            }
            datalist {
                id:"results",
                results.iter().map(|item| {
                    rsx! {
                        option {
                            value: "{item.id()}",
                            label: "{item.name()} {item.pn()}-{item.version():03}"
                        }
                    }
                })

            }
            button {
                class: "w3-button w3-theme",
                "Add child"
            }
        }
    }
}
