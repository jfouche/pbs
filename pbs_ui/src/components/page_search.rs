use crate::client;
use dioxus::prelude::*;
use futures_util::StreamExt;
use pbs_srv::Item;

pub fn page_search(cx: Scope) -> Element {
    let results: &UseState<Option<Vec<Item>>> = use_state(cx, || None);
    let message = use_state(cx, || "".to_string());

    let search_handler = use_coroutine(cx, |mut rx: UnboundedReceiver<String>| {
        to_owned![message, results];
        async move {
            while let Some(pattern) = rx.next().await {
                let result = client::search_items(&pattern).await;
                match result {
                    Ok(items) => {
                        message.set(format!("FOUND {} items", items.len()));
                        results.set(Some(items));
                    }
                    Err(e) => {
                        results.set(None);
                        message.set(format!("ERROR : {e:?}"))
                    }
                }
            }
        }
    });

    cx.render(rsx! {
        h2 { "Search item" },
        input {
            "value": "",
            oninput: move |evt| {
                let pattern = evt.value.to_owned();
                if pattern.len() > 2 {
                    search_handler.send(pattern);
                }
            },
            onmounted: move |evt| {evt.data.set_focus(true);},
        }

        match results.get() {
            Some(v) => rsx!( search_results { items: v } ),
            None => rsx!( tr { td { "collspan": 5, "Enter pattern" } } )
        }
        div { "{message}"}
    })
}

#[derive(Props)]
struct SearchResultsProps<'a> {
    items: &'a Vec<Item>,
}

fn search_results<'a>(cx: Scope<'a, SearchResultsProps<'a>>) -> Element {
    render!(
        h2 { "Results" },
        table {
            tr {
                th { "Name" },
                th { "Part number" },
                th { "Version" },
                th { "Maturity" },
                th { "Action" },
            }
            cx.props.items.iter().map(|i| rsx!( item_row { item: i }))
        }
    )
}

#[derive(Props)]
struct ItemRowProps<'a> {
    item: &'a Item,
}

fn item_row<'a>(cx: Scope<'a, ItemRowProps<'a>>) -> Element {
    render!(
        tr {
            td { cx.props.item.name() },
            td { cx.props.item.pn() },
            td { cx.props.item.version().to_string() },
            td { cx.props.item.maturity().to_string() },
            td {
                a {
                "View"
                },
            }
        }
    )
}
