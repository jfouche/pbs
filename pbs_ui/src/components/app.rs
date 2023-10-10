use dioxus::prelude::*;

use crate::components::page_new_item::PageNewItem;
use crate::components::page_search::PageSearch;
use crate::components::top_menu::TopMenu;

use crate::components::Page;

pub fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, || Page::SearchItems);
    let current_page = use_shared_state::<Page>(cx).unwrap();
    let current_page = *current_page.read();

    render! {
        div {
            style { include_str!("./assets/style.css") }
            TopMenu { },
            div {
                "Hello, world!"
            }
            div {
                class:"container",
                h1 { "Product Breakdow Software" }
                PageNewItem { }
                PageSearch { }
                // PageViewItem { }
                // match current_page {
                //     Page::NewItem => None,
                //     Page::SearchItems => PageSearch { },
                //     Page::ViewItem => None
                // }

            // <PageNewItem v-if="active_page === 'page_new_item'"/>
            // <PageSearchItem v-else-if="active_page === 'page_search_items'"  @item-selection="select_item"/>
            // <PageViewItem v-else-if="active_page === 'page_view_item'" :item="active_item"/>
            // <p v-else>UNKNOWN PAGE</p>
            }
        }
    }
}
