mod app;
mod page_new_item;
mod page_search;
mod page_view_item;
mod top_menu;

pub use app::App;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Page {
    NewItem,
    SearchItems,
    ViewItem,
}
