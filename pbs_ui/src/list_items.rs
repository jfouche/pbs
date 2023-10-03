use pbs_core::Item;

fn list_items_panel_ui(ui: &mut egui::Ui, items: &Vec<Item>) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 1.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    egui::ScrollArea::vertical().show(ui, |ui| {
        for item in items {
            ui.label(item.to_string());
        }
    });
    response
}

pub fn list_items_panel(items: &Vec<Item>) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| list_items_panel_ui(ui, items)
}
