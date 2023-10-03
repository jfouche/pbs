pub struct NewItem {
    cots: bool,
    pn: String,
    name: String,
}

impl NewItem {
    fn ui(&mut self, ui: &mut egui::Ui) -> egui::Response {
        ui.label("PN :");
        ui.text_edit_singleline(&mut self.pn);
        ui.label("Name :");
        ui.text_edit_singleline(&mut self.name);
        ui.button("Add item")
    }
}

pub fn new_item_panel(new_item: &mut NewItem) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| new_item.ui(ui)
}
