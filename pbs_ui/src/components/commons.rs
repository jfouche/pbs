use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::bs_icons::{
        BsCurrencyDollar, BsPatchCheck, BsPatchExclamation, BsPatchQuestion, BsTools,
    },
    Icon,
};
use pbs_srv::{Item, ItemMaturity, Strategy};
use tracing::info;

#[derive(Props, PartialEq)]
pub struct ItemProps {
    item: Item,
}

pub fn item_descr(cx: Scope<ItemProps>) -> Element {
    info!("item_descr({})", cx.props.item.id());
    render! {
        span {
            class: "item-desc",
            match cx.props.item.strategy() {
                Strategy::Make => rsx! { Icon { icon : BsTools } },
                Strategy::Buy => rsx! { Icon { icon : BsCurrencyDollar } }
            }
            span { cx.props.item.pn() }
            "-"
            span { format!("{:03}", cx.props.item.version()) }
            b { cx.props.item.name() }
            match cx.props.item.maturity() {
                ItemMaturity::InProgress => rsx! { Icon { icon: BsPatchQuestion } },
                ItemMaturity::Released => rsx! { Icon { icon: BsPatchCheck } },
                ItemMaturity::Obsolete => rsx! { Icon { icon: BsPatchExclamation } },
            }
        }
    }
}

#[derive(Props, PartialEq)]
pub struct QuantityProps {
    quantity: usize,
}

pub fn item_quantity(cx: Scope<QuantityProps>) -> Element {
    render! {
        span {
            class: "item-quantity",
            format!("{}", cx.props.quantity),
        }
    }
}
