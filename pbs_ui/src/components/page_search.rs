use dioxus::prelude::*;
use pbs_srv::Item;

use crate::client;

pub fn page_search(cx: Scope) -> Element {
    use_shared_state_provider(cx, || SearchState::Unset);

    let pattern = use_state(cx, || "".to_string());
    let results: &UseState<Option<Vec<Item>>> = use_state(cx, || None);

    cx.render(rsx! {
        h2 { "Search item" },
        input {
            "value": "{pattern}",
            oninput: |e| { resolve_search(pattern.get(), results.clone()); }
        }
        h2 { "Results" },
        table {
            tr {
                th { "Name" },
                th { "Part number" },
                th { "Version" },
                th { "Maturity" },
                th { "Action" },
            }
            match results.get() {
                Some(items) => { items.iter().map(|i| rsx!( item_row { item: i })); },
                None => {rsx!( tr { td { "collspan": 5, "Enter pattern" } } );}
            }

        }
    })
}

enum SearchState {
    Unset,
    Loading,
    Loaded(Vec<Item>),
}

#[derive(Props)]
struct ItemRowProps<'a> {
    item: &'a Item,
}

fn item_row<'a>(cx: Scope<'a, ItemRowProps<'a>>) -> Element {
    cx.render(rsx! {
        tr {
            td { cx.props.item.name() },
            td { cx.props.item.pn() },
            td { cx.props.item.version().to_string() },
            td { cx.props.item.maturity().to_string() },
            td { cx.props.item.name() },
        }
    })
}

async fn resolve_search(pattern: &String, search_results: UseState<Option<Vec<Item>>>) {
    if pattern.len() > 2 {
        match client::search_items(&pattern).await {
            Ok(items) => {
                search_results.set(Some(items));
            }
            Err(_) => {
                todo!();
            }
        }
    }
}
