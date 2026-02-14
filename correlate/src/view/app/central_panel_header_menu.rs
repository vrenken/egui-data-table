use crate::view::row_viewer::Viewer;

impl Viewer {
    pub fn ui_column_header_context_menu(&mut self, ui: &mut egui::Ui, column: usize) {
        let is_name_active = self.column_configs[column].is_name;
        let is_key_active = self.column_configs[column].is_key;

        (&mut*ui).separator(); // ========================================

        let mut is_key = is_key_active;
        if (&mut*ui).checkbox(&mut is_key, "Use as key").clicked() {
            self.column_configs[column].is_key = is_key;
            self.save_requested = true;
            ui.close();
        }

        let mut is_name = is_name_active;
        if (&mut*ui).checkbox(&mut is_name, "Use as name").clicked() {
            if is_name {
                // Turn off is_name for all other columns
                for c in self.column_configs.iter_mut() {
                    c.is_name = false;
                }
                self.column_configs[column].is_name = true;
            } else {
                self.column_configs[column].is_name = false;
            }
            self.save_requested = true;
            ui.close();
        }
        ui.separator();

        if (&mut*ui).button(format!("{} Insert left", egui_material_icons::icons::ICON_ADD_COLUMN_LEFT)).clicked() {
            self.add_column_requested = Some(column);
            ui.close();
        }
        if (&mut*ui).button(format!("{} Insert right", egui_material_icons::icons::ICON_ADD_COLUMN_RIGHT)).clicked() {
            self.add_column_requested = Some(column);
            ui.close();
        }


        ui.separator();

        if column > 0 {
            if ui.button("Move Left").clicked() {
                self.column_configs.swap(column, column - 1);
                self.save_requested = true;
                ui.close();
            }
        }
        if column < self.column_configs.len() - 1 {
            if ui.button("Move Right").clicked() {
                self.column_configs.swap(column, column + 1);
                self.save_requested = true;
                ui.close();
            }
        }
    }
}
