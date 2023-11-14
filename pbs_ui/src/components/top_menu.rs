use dioxus::prelude::*;
use dioxus_router::components::{GoBackButton, GoForwardButton, Link};

use crate::components::route::Route;

pub fn top_menu(cx: Scope) -> Element {
    render!(
        ul {
            id: "top-menu",
            GoBackButton { "<" }
            GoForwardButton  { ">" }
            menu_item { title: "New item", route: Route::NewItem { } }
            menu_item { title: "Search item", route: Route::Search { pattern: String::new() } }
        }
    )
}

#[derive(Props)]
struct MenuItemProps<'a> {
    title: &'a str,
    route: Route,
}

fn menu_item<'a>(cx: Scope<'a, MenuItemProps<'a>>) -> Element {
    let title = cx.props.title;
    render!(
        li {
            class: "",
            Link {
                to: cx.props.route.to_owned(),
                "{title}"
            }
        }
    )
}
