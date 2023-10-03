#[derive(Default)]
pub struct Search {
    pub pattern: String,
}

impl Search {
    fn ui(&mut self, ui: &mut egui::Ui) -> egui::Response {
        ui.label("item pattern :");
        ui.text_edit_singleline(&mut self.pattern);
        ui.button("search")
    }
}

pub fn search_panel(search: &mut Search) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| search.ui(ui)
}
