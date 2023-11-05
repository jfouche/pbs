use dioxus::prelude::*;
use futures_util::StreamExt;
use pbs_srv::{Child, Item};

use crate::client;

#[derive(Props, PartialEq)]
pub struct ItemIdProps {
    pub id: i64,
}

pub fn page_view_item(cx: Scope<ItemIdProps>) -> Element {
    let item = use_ref(cx, || Option::<Item>::None);
    let load_item_handler = load_item_handler(cx, item.to_owned());

    render! {
        div {
            onmounted: move |_| load_item_handler.send(cx.props.id),

            match *item.read() {
                Some(ref item) => rsx!(
                    h2 { "{item.name()}" }
                    tree_item { item : item.clone(), quantity : 1 }
                    // add_child {}
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
    let children = use_ref(cx, Vec::<Child>::new);

    // let children_ref = children.to_owned();

    let have_children = children.with(|c| c.is_empty());
    let open_text = is_open.with(|b| if *b { "-" } else { "+" });
    let show_children = have_children && *is_open.read();

    let load_children_handler = load_children_handler(cx, children.to_owned());

    let id = cx.props.item.id();
    let pn = cx.props.item.pn();
    let name = cx.props.item.name();
    let quantity = cx.props.quantity;

    println!(
        "tree_item#{id} have_children: {have_children}, open_text : {open_text}, show_children: {show_children}"
    );

    let toggle = move || {
        is_open.with_mut(|b| *b = !*b);
        if children.read().is_empty() {
            load_children_handler.send(id);
        }
    };

    render! {
        li {
            div {
                onclick: move |_| toggle(),
                "{pn} - {name} | quantity : {quantity}",
                if have_children {
                    rsx!(span { "{open_text}" })
                }

            }

            children.read().iter().map(|child| rsx! {
                tree_item {
                    item: child.item.clone(),
                    quantity: child.quantity
                }
            })

        }
    }
}

// #[derive(Props, PartialEq)]
// struct ChildrenProps {
//     items: Vec<(Item, usize)>,
// }

// fn children_lines(cx: Scope<ChildrenProps>) -> Element {
//     render! {
//         cx.props.items.into_iter().map(|child| rsx! {
//             tree_item {
//                 item: child.0.clone(),
//                 quantity: child.1
//             }
//         })
//     }
// }

fn load_item_handler(cx: &ScopeState, item: UseRef<Option<Item>>) -> &Coroutine<i64> {
    use_coroutine(cx, |mut rx: UnboundedReceiver<i64>| async move {
        while let Some(id) = rx.next().await {
            match client::item(id).await {
                Ok(i) => item.set(Some(i)),
                Err(_e) => {
                    todo!()
                }
            }
        }
    })
}

fn load_children_handler(cx: &ScopeState, children: UseRef<Vec<Child>>) -> &Coroutine<i64> {
    use_coroutine(cx, |mut rx: UnboundedReceiver<i64>| async move {
        while let Some(id) = rx.next().await {
            match client::children(id).await {
                Ok(c) => {
                    println!("received children : {}", c.len());
                    children.set(c.into_iter().map(|c| c.into()).collect());
                }
                Err(_e) => {
                    todo!()
                }
            }
        }
    })
}
