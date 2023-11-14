use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::components::panel_new_item::panel_new_item;
use crate::components::panel_recently_use::panel_recently_use;
use crate::components::panel_search::panel_search;
use crate::components::panel_view_item::panel_view_item;
use crate::components::top_menu::top_menu;

#[derive(Routable, Clone)]
pub enum Route {
    // The home page is at the / route
    #[route("/")]
    // If the name of the component and variant are the same you can omit the component and props name
    // If they are different you can specify them like this:
    // #[route("/", ComponentName, PropsName)]
    Home {},

    #[route("/search/:pattern")]
    Search { pattern: String },

    #[route("/new_item")]
    NewItem {},

    #[route("/item/:id")]
    ViewItem { id: i64 },

    // PageNotFound is a catch all route that will match any route and placing the matched segments in the route field
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

fn Home(cx: Scope) -> Element {
    render! {
        app_nest {
            panel_search { pattern: String::new() }
        }
    }
}

#[inline_props]
fn Search(cx: Scope, pattern: String) -> Element {
    render! {
        app_nest {
            panel_search { pattern: pattern.to_owned() }
        }
    }
}

fn NewItem(cx: Scope) -> Element {
    render! {
        app_nest {
            panel_new_item { }
        }
    }
}

#[inline_props]
fn ViewItem(cx: Scope, id: i64) -> Element {
    render! {
        app_nest {
            panel_view_item { id: *id }
        }
    }
}

#[inline_props]
fn PageNotFound(cx: Scope, route: Vec<String>) -> Element {
    render! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre {
            color: "red",
            "log:\nattemped to navigate to: {route:?}"
        }
    }
}

#[inline_props]
fn app_nest<'a>(cx: Scope, children: Element<'a>) -> Element {
    render! {
        div {
            style { include_str!("./assets/style.css") }
            top_menu { },
            div {
                style: "display: flex",
                div {
                    style: "flex:1",
                    class:"container",
                    h1 { "Product Breakdow Software" }
                    children
                },
                panel_recently_use { },
            }
        }
    }
}
