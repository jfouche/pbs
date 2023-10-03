#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use list_items::list_items_panel;
use new_item::{new_item_panel, NewItem};
use pbs_core::{Item, Store};
use search::{search_panel, Search};

mod list_items;
mod new_item;
mod search;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

struct MyApp {
    store: Store,
    search: Search,
    //    new_item: NewItem,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            store: Store::open("store.db3").unwrap(),
            ..Default::default()
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // let mut items = vec![];
        // if self.search.pattern.len() >= 3 {
        //     items = self.store.search_items(&self.search.pattern).unwrap();
        // }

        // Side panel
        // egui::SidePanel::left("LEFT PANEL").show(ctx, |ui| {
        //     ui.heading("LEFT PANEL");
        //     // ui.add(search_panel(&mut self.search));
        //     // ui.add(list_items_panel(&items));
        // });

        // Central panel
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
        });

        // egui::TopBottomPanel::bottom("BOTTOM PANEL").show(ctx, |ui| {
        //     ui.heading("BOTTOM PANEL");
        //     // ui.add(new_item_panel(&mut self.new_item)).clicked();
        // });

        // if self.search.pattern.len() >= 3 {}
    }
}
