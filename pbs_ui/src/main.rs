#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::{ScrollArea, Sense, Vec2};
use pbs_core::{Item, Store};

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
    search_pattern: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            store: Store::open("store.db3").unwrap(),
            search_pattern: String::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let items = self.store.get_items().unwrap();

        // Side panel
        egui::SidePanel::left("SIDE PANEL").show(ctx, |ui| {
            ui.heading("search item");
            ui.text_edit_singleline(&mut self.search_pattern);
            ui.add(search_panel(&items));
        });

        // Central panel
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            // ui.horizontal(|ui| {
            //     let name_label = ui.label("Your name: ");
            //     ui.text_edit_singleline(&mut self.name)
            //         .labelled_by(name_label.id);
            // });
            // ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            // if ui.button("Click each year").clicked() {
            //     self.age += 1;
            // }
            // ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
    }
}

fn search_panel_ui(ui: &mut egui::Ui, items: &Vec<Item>) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 1.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    ScrollArea::vertical().show(ui, |ui| {
        for item in items {
            ui.label(item.to_string());
        }
    });
    response
}

fn search_panel(items: &Vec<Item>) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| search_panel_ui(ui, items)
}
