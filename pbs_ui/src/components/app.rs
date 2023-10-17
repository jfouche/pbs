use dioxus::prelude::*;

use crate::components::page_new_item::page_new_item;
use crate::components::page_search::page_search;
use crate::components::page_view_item::page_view_item;
use crate::components::top_menu::top_menu;

use crate::components::Page;

pub fn app(cx: Scope) -> Element {
    use_shared_state_provider(cx, || Page::SearchItems);
    let current_page = use_shared_state::<Page>(cx).unwrap();
    let current_page = *current_page.read();

    render! {
        div {
            style { include_str!("./assets/style.css") }
            top_menu { },
            div {
                "Hello, world!"
            }
            div {
                class:"container",
                h1 { "Product Breakdow Software" }

                match current_page {
                    Page::NewItem => rsx!( page_new_item { } ),
                    Page::SearchItems => rsx!( page_search { } ),
                    Page::ViewItem => rsx!( page_view_item { } )
                }
            }
        }
    }
}
