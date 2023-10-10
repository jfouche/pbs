use dioxus::prelude::*;
use pbs_srv::Item;

pub fn PageViewItem(cx: Scope) -> Element {
    render! {
        div {
            class: "tree",
            h2 {"PN / NAME"}
            // TreeItem {}
            // AddChild {}
            "Search item"
        }
    }
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

fn ItemRow<'a>(cx: Scope<'a, ItemRowProps<'a>>) -> Element {
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

// async fn resolve_search<'a>(
//     search_results: UseRef<Option<&'a Vec<Item>>>,
//     preview_state: UseSharedState<SearchState>,
//     story_id: i64,
// ) {
//     if let Some(cached) = *search_results.read() {
//         *preview_state.write() = SearchState::Loaded(*cached);
//         return;
//     }

//     *preview_state.write() = SearchState::Loading;
//     if let Ok(items) = client::search_items().await {
//         *preview_state.write() = SearchState::Loaded(story);
//         *full_story.write() = Some(story);
//     }
// }
