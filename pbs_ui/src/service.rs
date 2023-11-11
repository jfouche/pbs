use dioxus::prelude::*;
use futures_util::StreamExt;
use pbs_srv::{Child, Item};

use crate::client;

/// Service that poll for pattern. One a paatern is available, it
/// search for [Item]s matching this pattern, and update the results
pub async fn search_service(
    mut rx: UnboundedReceiver<String>,
    results: UseState<Vec<Item>>,
    message: UseState<String>,
) {
    while let Some(pattern) = rx.next().await {
        match client::search_items(&pattern).await {
            Ok(items) => {
                message.set(format!("FOUND {} items", items.len()));
                results.set(items);
            }
            Err(e) => {
                results.set(vec![]);
                message.set(format!("ERROR : {e:?}"))
            }
        }
    }
}

///
pub async fn make_item_service(mut rx: UnboundedReceiver<String>, message: UseState<String>) {
    while let Some(name) = rx.next().await {
        let msg = match client::item_make(&name).await {
            Ok(item) => format!("Item MAKE [{}] created", item.pn()),
            Err(e) => format!("ERROR : {e:?}"),
        };
        message.set(msg);
    }
}

///
pub async fn buy_item_service(
    mut rx: UnboundedReceiver<(String, String)>,
    message: UseState<String>,
) {
    while let Some((pn, name)) = rx.next().await {
        let msg = match client::item_buy(&pn, &name).await {
            Ok(_) => "Item BUY created".to_string(),
            Err(e) => format!("ERROR : {e:?}"),
        };
        message.set(msg);
    }
}

///
pub async fn load_item_service(id: i64) -> Item {
    match client::item(id).await {
        Ok(item) => item,
        Err(e) => {
            eprint!("ERROR : {e:?}");
            todo!()
        }
    }
}

///
pub async fn load_children_service(
    mut rx: UnboundedReceiver<i64>,
    children: UseRef<Option<Vec<Child>>>,
) {
    while let Some(id) = rx.next().await {
        match client::children(id).await {
            Ok(c) => {
                println!("load_children_handler() - received {} children", c.len());
                children.set(Some(c.into_iter().map(|c| c.into()).collect()));
            }
            Err(e) => {
                eprint!("ERROR : {e:?}");
                todo!()
            }
        }
    }
}
