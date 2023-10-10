use dioxus::prelude::*;

use super::Page;

pub fn TopMenu(cx: Scope) -> Element {
    cx.render(rsx! {
        ul {
            id: "top-menu",
            MenuItem { title: "New item", page: Page::NewItem}
            MenuItem { title: "Search item", page: Page::SearchItems}
            MenuItem { title: "View item", page: Page::ViewItem}
        }
    })
}

#[derive(Props)]
struct MenuItemProps<'a> {
    title: &'a str,
    page: Page,
}

fn MenuItem<'a>(cx: Scope<'a, MenuItemProps<'a>>) -> Element {
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
