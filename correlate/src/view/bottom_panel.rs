use crate::view::*;


#[derive(Default)]
pub struct BottomPanel {}

impl BottomPanel {
    pub fn ui(&mut self, view_model: &mut RootViewModel, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            egui::Sides::new().show(ui, |_ui| {
            }, |ui|{
                let mut has_modifications = view_model.table.has_user_modification();
                ui.add_enabled(false, egui::Checkbox::new(&mut has_modifications, "Has modifications"));

                ui.add_enabled_ui(has_modifications, |ui| {
                    if ui.button("Clear").clicked() {
                        view_model.table.clear_user_modification_flag();
                    }
                });
            });
        });
    }
}
