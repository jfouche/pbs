use dioxus::prelude::*;

use super::Page;

pub fn top_menu(cx: Scope) -> Element {
    cx.render(rsx! {
        ul {
            id: "top-menu",
            menu_item { title: "New item", page: Page::NewItem}
            menu_item { title: "Search item", page: Page::SearchItems}
            menu_item { title: "View item", page: Page::ViewItem}
        }
    })
}

#[derive(Props)]
struct MenuItemProps<'a> {
    title: &'a str,
    page: Page,
}

fn menu_item<'a>(cx: Scope<'a, MenuItemProps<'a>>) -> Element {
    let current_page = use_shared_state::<Page>(cx).unwrap();
    let title = cx.props.title;
    let page = cx.props.page;
    let class = if *current_page.read() == page {
        "active"
    } else {
        ""
    };

    cx.render(rsx! {
        li {
            class: class,
            a {
                onclick: move |e| {*current_page.write() = page; e.stop_propagation(); },
                "{title}"
            }
        }
    })
}
