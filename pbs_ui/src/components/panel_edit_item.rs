use dioxus::prelude::*;

use pbs_srv::Item;
use tracing::warn;

use crate::{
    components::commons::{item_descr, item_quantity},
    service::{
        add_child_service, delete_child_service, load_children_service, load_item_service,
        search_coroutine,
    },
};

use super::{ItemIdChangeProps, ItemIdProps};

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
                                        button {
                                            class: "w3-button w3-theme",
                                            onclick: move |_| {
                                                delete_child_service(cx, item.id(), child.id());
                                                children_future.restart();
                                            },
                                            "Delete"
                                        }
                                    }
                                })
                             }
                        }
                    }
                    hr {}
                    panel_update {
                        id: cx.props.id,
                        on_change: move |_| { warn!("YOUYOU - 1"); children_future.restart(); }
                    }
                ),
                None => rsx!(p { "loading" })
            }
        }
    }
}

fn panel_update<'a>(cx: Scope<'a, ItemIdChangeProps<'a>>) -> Element {
    let results = use_state(cx, Vec::<Item>::new);
    let message = use_state(cx, String::new);
    let search_handler = use_coroutine(cx, |rx| {
        search_coroutine(rx, results.to_owned(), message.to_owned())
    });

    render! {
        div {
            input {
                class: "w3-border w3-padding",
                oninput: move |evt| {
                    let pattern = evt.value.to_owned();
                    if pattern.len() > 2 {
                        search_handler.send(pattern);
                    }
                }
            }
        }
        match results.len() {
            0 => rsx!( "Enter pattern" ),
            _ => rsx!( search_results {
                parent_id: cx.props.id,
                items: results,
                on_child_added: move |_| { warn!("YOUYOU - 2"); cx.props.on_change.call(());}
            }),
        }
        div { "{message}"}
    }
}

#[derive(Props)]
struct SearchResultsProps<'a> {
    parent_id: i64,
    items: &'a Vec<Item>,
    on_child_added: EventHandler<'a>,
}

fn search_results<'a>(cx: Scope<'a, SearchResultsProps<'a>>) -> Element {
    render!(
        div {
            style: "display: grid; width: 100%; grid-template-columns: 1fr 100px 80px;",
            cx.props.items.iter().map(|item| rsx! {
                item_row {
                    parent_id: cx.props.parent_id,
                    item: item,
                    on_child_added: move |_| {warn!("YOUYOU - 3"); cx.props.on_child_added.call(());}
                }
            })
        }
    )
}

#[derive(Props)]
struct ItemRowProps<'a> {
    parent_id: i64,
    item: &'a Item,
    on_child_added: EventHandler<'a>,
}

fn item_row<'a>(cx: Scope<'a, ItemRowProps<'a>>) -> Element {
    let quantity = use_state(cx, || 1);
    let added = use_state(cx, || false);

    if *added.get() {
        cx.props.on_child_added.call(());
        added.set(false);
    }

    render!(
        div {
            item_descr { item: cx.props.item.clone() }
        }
        div {
            input {
                r#type: "number",
                value: "{quantity}",
                oninput: move |evt| quantity.set(evt.value.parse::<usize>().unwrap()),

            }
        }
        div {
            button {
                class: "w3-button w3-theme",
                onclick: move |_| {
                    add_child_service(cx, cx.props.parent_id, cx.props.item.id(), **quantity, added.to_owned());
                },
                "Add"
            }
        }
    )
}
