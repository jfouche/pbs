mod app;
mod commons;
mod panel_edit_item;
mod panel_new_item;
mod panel_recently_use;
mod panel_search;
mod panel_view_item;
mod route;
mod top_menu;

pub use app::app;
use dioxus::{core::prelude::EventHandler, core_macro::Props};
use pbs_srv::Item;

// COMMON PROPS

#[derive(Props, PartialEq)]
pub struct ItemIdProps {
    pub id: i64,
}

#[derive(Props, PartialEq)]
pub struct ItemRefProps<'a> {
    item: &'a Item,
}

#[derive(Props)]
pub struct ItemIdChangeProps<'a, T = ()> {
    pub id: i64,
    pub on_change: EventHandler<'a, T>,
}
