use crate::view::row_viewer::Viewer;
use crate::view::app::types::RenamingTarget;

impl Viewer {
    pub fn ui_column_header_context_menu(
        &mut self,
        ui: &mut egui::Ui,
        column: usize) {


        ui.horizontal(|ui| {
            ui.label(egui_material_icons::icons::ICON_NOTES);

            let (_, current_name) = self.renaming_item.get_or_insert_with(|| {
                let config = &self.column_configs[column];
                let display_name = config.display_name.as_ref().unwrap_or(&config.name).clone();
                (RenamingTarget::Column(column), display_name)
            });

            let res = ui.text_edit_singleline(current_name);
            if res.lost_focus() || ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                self.rename_committed = true;
                ui.close();
            }
            if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                self.renaming_item = None;
                ui.close();
            }
            res.request_focus();

        });

        let is_name_active = self.column_configs[column].is_name;
        let is_key_active = self.column_configs[column].is_key;

        (&mut*ui).separator(); // ========================================

        let mut is_key = is_key_active;
        if (&mut*ui).checkbox(&mut is_key, "Use as key").clicked() {
            self.column_configs[column].is_key = is_key;
            self.save_requested = true;
            ui.close();
        }

        if (&mut*ui).button(format!("{} Filter", egui_material_icons::icons::ICON_FILTER_LIST)).clicked() {
            ui.close();
        }
        (&mut*ui).menu_button(format!("{} Sort", egui_material_icons::icons::ICON_SWAP_VERT), |ui| {
            if (&mut*ui).button(format!("{} Sort ascending", egui_material_icons::icons::ICON_NORTH)).clicked() {
                ui.close();
            }
            if (&mut*ui).button(format!("{} Sort descending", egui_material_icons::icons::ICON_SOUTH)).clicked() {
                ui.close();
            }

        });
        if (&mut*ui).button(format!("{} Hide", egui_material_icons::icons::ICON_VISIBILITY_OFF)).clicked() {
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

        ui.separator();

        if (&mut*ui).button(format!("{} Duplicate", egui_material_icons::icons::ICON_STACK)).clicked() {
            ui.close();
        }
        if (&mut*ui).button(format!("{} Trash", egui_material_icons::icons::ICON_DELETE)).clicked() {
            ui.close();
        }

    }
}
