#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;

use pbs_core::Store;
use tauri::State;

struct StoreState(Mutex<Store>);

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn create_item(name: &str, store: State<StoreState>) -> Result<String, String> {
    match store.0.lock().unwrap().create(name) {
        Ok(item) => Ok(format!("Add {}, PN : {}", item.name(), item.pn())),
        Err(e) => Err(format!("{e:?}")),
    }
}

fn main() {
    let store = Store::open("store.db3").unwrap();
    tauri::Builder::default()
        .manage(StoreState(Mutex::new(store)))
        .invoke_handler(tauri::generate_handler![create_item])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
