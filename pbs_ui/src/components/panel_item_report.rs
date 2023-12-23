use dioxus::prelude::*;

use crate::{
    components::commons::{item_descr, item_quantity},
    service::{load_item_service, load_report_service},
};

use super::ItemIdProps;

pub fn panel_item_report(cx: Scope<ItemIdProps>) -> Element {
    let item_future = use_future(cx, (), |_| load_item_service(cx.props.id));
    let report_future = use_future(cx, (), |_| load_report_service(cx.props.id));

    render! {
        div {
            match item_future.value() {
                Some(item) => rsx!(
                    item_descr { item: item.clone() }
                ),
                None => rsx!(p { "loading item" })
            }
        }
        ul {
            match report_future.value() {
                Some(report) => rsx! {
                    report.into_iter().map(|child| {
                        rsx! {
                            li {
                                item_descr { item: child.item().clone() }
                                item_quantity { quantity: child.quantity() }
                            }
                        }
                    })
                },
                None => rsx! { "loading report" },
            }
        }
    }
}
