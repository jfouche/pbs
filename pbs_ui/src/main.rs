#![allow(non_snake_case)]
use tokio::runtime::Runtime;
use tracing::error;

mod client;
mod components;
mod service;

fn main() {
    let rt = Runtime::new().expect("Can't create Tokyio::Runtime");
    rt.block_on(async move {
        tracing_subscriber::fmt::init();

        // launch the PBS server
        tokio::spawn(async {
            if let Err(e) = pbs_srv::serve(3030).await {
                error!("ERROR {e:?}");
            }
        });
        // launch the dioxus app in a webview
        dioxus_desktop::launch(components::app);
    });
}
