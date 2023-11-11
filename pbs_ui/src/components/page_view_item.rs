use dioxus::prelude::*;
use pbs_srv::{Child, Item};

use crate::{
    components::commons::{item_descr, item_quantity},
    service::{load_children_service, load_item_service},
};

#[derive(Props, PartialEq)]
pub struct ItemIdProps {
    pub id: i64,
}

pub fn page_view_item(cx: Scope<ItemIdProps>) -> Element {
    let item_future = use_future(cx, (), |_| load_item_service(cx.props.id));

    render! {
        div {
            id: "view-item",
            match item_future.value() {
                Some(item) => rsx!(
                    h2 { item.name() }
                    ul {
                        tree_item { item : item.clone(), quantity : 1 }
                    }
                ),
                None => rsx!(p { "loading" })
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
        use_coroutine(cx, |rx| load_children_service(rx, children.to_owned()));

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
