use crate::components::route::Route;
use crate::{components::commons::item_descr, service::search_service};
use dioxus::prelude::*;
use dioxus_router::components::Link;
use pbs_srv::Item;

#[inline_props]
pub fn panel_search(cx: Scope, pattern: String) -> Element {
    let results = use_state(cx, Vec::<Item>::new);
    let message = use_state(cx, String::new);

    let search_handler = use_coroutine(cx, |rx| {
        search_service(rx, results.to_owned(), message.to_owned())
    });

    render! {
        h2 { "Search item" },
        input {
            "value": "{pattern}",
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
    render!(
        li {
            item_descr { item: cx.props.item.clone() },
            Link {
                to: Route::ViewItem { id: cx.props.item.id() },
                "View"
            },
        }
    )
}
