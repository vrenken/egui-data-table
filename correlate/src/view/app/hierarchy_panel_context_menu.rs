use crate::view::CorrelateApp;

impl CorrelateApp {
    pub fn ui_hierarchy_panel_context_menu(&mut self, ui: &mut egui::Ui) {
        if ui.button("Add").clicked() {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("Excel Files", &["xlsx"])
                .add_filter("CSV Files", &["csv"])
                .pick_file() 
            {
                self.pending_file_to_add = Some(path);
            }
            ui.close();
        }
    }
}
