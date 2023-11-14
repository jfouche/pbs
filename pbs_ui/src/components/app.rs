use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::components::route::Route;

pub fn app(cx: Scope) -> Element {
    render! {
        style { include_str!("./assets/w3.css") }
        style { include_str!("./assets/w3-theme-blue-grey.css") }
        style { include_str!("./assets/Open-Sans.css") }
        style { include_str!("./assets/font-awesome.min.css") }
        style { include_str!("./assets/style.css") }
        Router::<Route> {}
    }
}
