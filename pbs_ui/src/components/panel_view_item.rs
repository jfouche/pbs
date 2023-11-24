use dioxus::prelude::*;
use dioxus_router::components::Link;
use pbs_srv::{Child, Item};

use crate::{
    components::{
        commons::{item_descr, item_quantity},
        route::Route,
    },
    service::{load_children_coroutine, load_item_service, search_coroutine},
};

use super::{ItemIdProps, ItemRefProps};

pub fn panel_view_item(cx: Scope<ItemIdProps>) -> Element {
    let item_future = use_future(cx, (), |_| load_item_service(cx.props.id));

    render! {
        div {
            id: "view-item",
            match item_future.value() {
                Some(item) => rsx!(
                    item_title { item: item }
                    ul {
                        tree_item { item : item.clone(), quantity : 1 }
                    }
                    panel_update { }
                ),
                None => rsx!(p { "loading" })
            }
        }
    }
}

fn item_title<'a>(cx: Scope<'a, ItemRefProps<'a>>) -> Element {
    let item = cx.props.item;
    render!(
            h2 {
            "Item : {item.name()}"
            if item.is_modifiable() {
                rsx! {
                    Link {
                        class: "w3-button w3-theme",
                        to: Route::EditItem { id: item.id() },
                        "Create"                    }
                }
            }
        }
    )
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

#[derive(Props, PartialEq)]
struct TreeItemProps {
    item: Item,
    quantity: usize,
}

fn tree_item(cx: Scope<TreeItemProps>) -> Element {
    let is_open = use_ref(cx, || false);
    let children = use_ref(cx, || Option::<Vec<Child>>::None);
    let load_children_handler =
        use_coroutine(cx, |rx| load_children_coroutine(rx, children.to_owned()));

    let id = cx.props.item.id();
    let current_class = match children.read().as_ref() {
        None => "caret",
        Some(c) if c.is_empty() => "caret invisible",
        _ => is_open.with(|b| if *b { "caret caret-down" } else { "caret" }),
    };
    let children_class = is_open.with(|b| if *b { "nested active" } else { "nested" });

    let toggle = move || {
        is_open.with_mut(|b| *b = !*b);
        if children.read().is_none() {
            load_children_handler.send(id);
        }
    };

    render! {
        li {
            span {
                class: current_class,
                onclick: move |_| toggle(),
                item_descr { item: cx.props.item.clone() },
                item_quantity { quantity: cx.props.quantity }
            }
            children.read().as_ref().map(|c| rsx! {
                ul {
                    class: children_class,
                    c.iter().map(|child| rsx! {
                        tree_item {
                            item: child.item.clone(),
                            quantity: child.quantity
                        }
                    })
                }
            })
        }
    }
}
