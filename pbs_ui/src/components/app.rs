use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::components::route::Route;

pub fn app(cx: Scope) -> Element {
    render! {
        Router::<Route> {}
    }
}
