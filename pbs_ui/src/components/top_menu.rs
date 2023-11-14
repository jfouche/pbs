use dioxus::prelude::*;
use dioxus_router::components::{GoBackButton, GoForwardButton, Link};

use crate::components::route::Route;

pub fn top_menu(cx: Scope) -> Element {
    render!(
        ul {
            id: "top-menu",
            class: "w3-top w3-bar w3-theme-d2 w3-left-align w3-large",
            li {
                class: "w3-bar-item w3-button w3-hide-small w3-padding-large w3-hover-white",
                GoBackButton { "<" }

            }
            li {
                class: "w3-bar-item w3-button w3-hide-small w3-padding-large w3-hover-white",
                GoForwardButton  { ">" }
            }
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
            class: "w3-bar-item w3-button w3-hide-small w3-padding-large w3-hover-white",
            Link {
                to: cx.props.route.to_owned(),
                "{title}"
            }
        }
    )
}
