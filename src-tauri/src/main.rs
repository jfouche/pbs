#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;

use pbs_core::{Item, Store};
use serde::Serialize;
use tauri::State;

struct StoreState(Mutex<Store>);

#[derive(Serialize)]
struct AppItem {
    id: usize,
    pn: String,
    name: String,
}

impl From<&Item> for AppItem {
    fn from(item: &Item) -> Self {
        AppItem {
            id: item.id(),
            pn: item.pn().into(),
            name: item.name().into(),
        }
    }
}

#[tauri::command]
fn create_item(name: &str, store: State<StoreState>) -> Result<String, String> {
    match store.0.lock().unwrap().create(name) {
        Ok(item) => Ok(format!("Add {}, PN : {}", item.name(), item.pn())),
        Err(e) => Err(format!("{e:?}")),
    }
}

#[tauri::command]
fn import_item(pn: &str, name: &str, store: State<StoreState>) -> Result<String, String> {
    match store.0.lock().unwrap().new_item(pn, name) {
        Ok(item) => Ok(format!("Add {}, PN : {}", item.name(), item.pn())),
        Err(e) => Err(format!("{e:?}")),
    }
}

#[tauri::command]
fn search_items(pattern: &str, store: State<StoreState>) -> Result<Vec<AppItem>, String> {
    match store.0.lock().unwrap().search_items(pattern) {
        Ok(items) => Ok(items.iter().map(AppItem::from).collect::<Vec<_>>()),
        Err(e) => Err(format!("{e:?}")),
    }
}

#[tauri::command]
fn get_item_by_id(id: usize, store: State<StoreState>) -> Result<AppItem, String> {
    match store.0.lock().unwrap().get_item_by_id(id) {
        Ok(item) => Ok(AppItem::from(&item)),
        Err(e) => Err(format!("{e:?}")),
    }
}

fn main() {
    let store = Store::open("store.db3").unwrap();
    tauri::Builder::default()
        .manage(StoreState(Mutex::new(store)))
        .invoke_handler(tauri::generate_handler![
            create_item,
            import_item,
            search_items,
            get_item_by_id
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
