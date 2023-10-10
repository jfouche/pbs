#![allow(non_snake_case)]
use tokio::runtime::Runtime;

mod client;
mod components;

fn main() {
    println!("main()");

    let rt = Runtime::new().unwrap();
    rt.block_on(async move {
        // launch the PBS server
        tokio::spawn(async {
            pbs_srv::serve(3030).await;
        });
        // launch the dioxus app in a webview
        dioxus_desktop::launch(components::App);
    });
}
