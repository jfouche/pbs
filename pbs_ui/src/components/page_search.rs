use crate::{
    components::{commons::item_descr, Page},
    service::search_service,
};
use dioxus::prelude::*;
use pbs_srv::Item;

pub fn page_search(cx: Scope) -> Element {
    let results: &UseState<Vec<Item>> = use_state(cx, Vec::new);
    let message = use_state(cx, String::new);

    let search_handler = use_coroutine(cx, |rx| {
        search_service(rx, results.to_owned(), message.to_owned())
    });

    render! {
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

        match results.len() {
            0 => rsx!( tr { td { "collspan": 5, "Enter pattern" } } ),
            _ => rsx!( search_results { items: results } ),
        }
        div { "{message}"}
    }
}

#[derive(Props)]
struct SearchResultsProps<'a> {
    items: &'a Vec<Item>,
}

fn search_results<'a>(cx: Scope<'a, SearchResultsProps<'a>>) -> Element {
    render!(
        h2 { "Results" },
        ul {
            cx.props.items.iter().map(|i| rsx!( item_row { item: i }))
        }
    )
}

#[derive(Props)]
struct ItemRowProps<'a> {
    item: &'a Item,
}

fn item_row<'a>(cx: Scope<'a, ItemRowProps<'a>>) -> Element {
    let current_page = use_shared_state::<Page>(cx).unwrap();
    let id = cx.props.item.id();

    render!(
        li {
            item_descr { item: cx.props.item.clone() },
           a {
                onclick: move |e| {
                    *current_page.write() = Page::ViewItem(id);
                    e.stop_propagation();
                },
                "View"
            },
        }
    )
}
