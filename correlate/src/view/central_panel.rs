use eframe::emath::Align;
use egui::Layout;
use egui::scroll_area::ScrollBarVisibility;
use crate::view::root_view_model::RootViewModel;
use crate::data::RenamingTarget;
use crate::view::CentralPanelViewModel;

#[derive(Default)]
pub struct CentralPanel {}

impl CentralPanel {
    pub fn ui(&mut self,
              view_model: &mut RootViewModel,
              central_panel_view_model: &mut CentralPanelViewModel,
              ctx: &egui::Context) {
        // Sync renaming state to viewer
        view_model.viewer.renaming_item = view_model.renaming_item.clone();

        egui::CentralPanel::default()
            .show(ctx, |ui| {

            ui.vertical(|ui| {

                ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                    if ui.button(egui_material_icons::icons::ICON_PAGE_INFO).clicked() {}
                    if ui.button(egui_material_icons::icons::ICON_SWAP_VERT).clicked() {}
                    if ui.button(egui_material_icons::icons::ICON_FILTER_LIST).clicked() {}
                });

                match view_model.scroll_bar_always_visible {
                    true => {
                        ui.style_mut().spacing.scroll = egui::style::ScrollStyle::solid();
                        view_model.style_override.scroll_bar_visibility = ScrollBarVisibility::AlwaysVisible;
                    },
                    false => {
                        ui.style_mut().spacing.scroll = egui::style::ScrollStyle::floating();
                        view_model.style_override.scroll_bar_visibility = ScrollBarVisibility::VisibleWhenNeeded;
                    }
                };

                //let available = ui.available_size();

                ui.add(
                    //available,
                    egui_data_table::Renderer::new(&mut view_model.table, &mut view_model.viewer).with_style(view_model.style_override),
                );

                // Sync renaming state back from viewer
                view_model.renaming_item = view_model.viewer.renaming_item.clone();

                central_panel_view_model.handle_viewer_requests(view_model);
            });
        });
    }

    pub fn ui_row_context_menu(viewer: &mut crate::view::RowView, ui: &mut egui::Ui, column: usize) {
        if let Some(config) = viewer.column_configs.get_mut(column) {
            let mut is_key = config.is_key;
            if ui.checkbox(&mut is_key, "Is key").clicked() {
                config.is_key = is_key;
                // Reset the table to force a redraw with new header names
                ui.ctx().memory_mut(|_mem| {
                    // This is a hacky way to force a full redraw of the table
                    // by clearing its UI state cache if we had access to the ID.
                    // Since we don't easily have the ID here, we just hope the change
                    // is picked up on next frame.
                });
                ui.close();
            }
        }
    }

    pub fn ui_column_header_context_menu(
        viewer: &mut crate::view::RowView,
        ui: &mut egui::Ui,
        column: usize) {


        ui.horizontal(|ui| {
            ui.label(egui_material_icons::icons::ICON_NOTES);

            let (_, current_name) = viewer.renaming_item.get_or_insert_with(|| {
                let config = &viewer.column_configs[column];
                let display_name = config.display_name.as_ref().unwrap_or(&config.name).clone();
                (RenamingTarget::Column(column), display_name)
            });

            let res = ui.text_edit_singleline(current_name);
            if res.lost_focus() || ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                viewer.rename_committed = true;
                ui.close();
            }
            if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                viewer.renaming_item = None;
                ui.close();
            }
            res.request_focus();

        });

        let is_name_active = viewer.column_configs[column].is_name;
        let is_key_active = viewer.column_configs[column].is_key;

        ui.separator(); // ========================================

        let mut is_key = is_key_active;
        if ui.checkbox(&mut is_key, "Use as key").clicked() {
            viewer.column_configs[column].is_key = is_key;
            viewer.save_requested = true;
            ui.close();
        }

        if ui.button(format!("{} Filter", egui_material_icons::icons::ICON_FILTER_LIST)).clicked() {
            ui.close();
        }
        ui.menu_button(format!("{} Sort", egui_material_icons::icons::ICON_SWAP_VERT), |ui| {
            if ui.button(format!("{} Sort ascending", egui_material_icons::icons::ICON_NORTH)).clicked() {
                ui.close();
            }
            if ui.button(format!("{} Sort descending", egui_material_icons::icons::ICON_SOUTH)).clicked() {
                ui.close();
            }

        });
        if ui.button(format!("{} Hide", egui_material_icons::icons::ICON_VISIBILITY_OFF)).clicked() {
            ui.close();
        }

        let mut is_name = is_name_active;
        if ui.checkbox(&mut is_name, "Use as name").clicked() {
            if is_name {
                // Turn off is_name for all other columns
                for c in viewer.column_configs.iter_mut() {
                    c.is_name = false;
                }
                viewer.column_configs[column].is_name = true;
            } else {
                viewer.column_configs[column].is_name = false;
            }
            viewer.save_requested = true;
            ui.close();
        }
        ui.separator();

        if ui.button(format!("{} Insert left", egui_material_icons::icons::ICON_ADD_COLUMN_LEFT)).clicked() {
            viewer.add_column_requested = Some(column);
            ui.close();
        }
        if ui.button(format!("{} Insert right", egui_material_icons::icons::ICON_ADD_COLUMN_RIGHT)).clicked() {
            viewer.add_column_requested = Some(column);
            ui.close();
        }


        ui.separator();

        if column > 0 {
            if ui.button("Move Left").clicked() {
                viewer.column_configs.swap(column, column - 1);
                viewer.save_requested = true;
                ui.close();
            }
        }
        if column < viewer.column_configs.len() - 1 {
            if ui.button("Move Right").clicked() {
                viewer.column_configs.swap(column, column + 1);
                viewer.save_requested = true;
                ui.close();
            }
        }

        ui.separator();

        if ui.button(format!("{} Duplicate", egui_material_icons::icons::ICON_STACK)).clicked() {
            ui.close();
        }
        if ui.button(format!("{} Trash", egui_material_icons::icons::ICON_DELETE)).clicked() {
            ui.close();
        }

    }
}
