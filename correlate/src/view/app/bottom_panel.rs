use crate::view::CorrelateApp;

impl CorrelateApp {
    pub fn ui_bottom_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            egui::Sides::new().show(ui, |_ui| {
            }, |ui|{
                let mut has_modifications = self.table.has_user_modification();
                ui.add_enabled(false, egui::Checkbox::new(&mut has_modifications, "Has modifications"));

                ui.add_enabled_ui(has_modifications, |ui| {
                    if ui.button("Clear").clicked() {
                        self.table.clear_user_modification_flag();
                    }
                });
            });
        });
    }
}
