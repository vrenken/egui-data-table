use eframe::emath::Align;
use egui::Layout;
use egui::scroll_area::ScrollBarVisibility;
use egui_data_table::RowViewer;
use crate::view::CorrelateApp;
use crate::view::app::types::RenamingTarget;
use crate::data::CellValue;

#[derive(Default)]
pub struct CentralPanel {}

impl CentralPanel {
    pub fn ui(&mut self, app: &mut CorrelateApp, ctx: &egui::Context) {
        // Sync renaming state to viewer
        app.viewer.renaming_item = app.renaming_item.clone();

        egui::CentralPanel::default().show(ctx, |ui| {

            ui.vertical(|ui| {

                ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                    if ui.button(egui_material_icons::icons::ICON_PAGE_INFO).clicked() {}
                    if ui.button(egui_material_icons::icons::ICON_SWAP_VERT).clicked() {}
                    if ui.button(egui_material_icons::icons::ICON_FILTER_LIST).clicked() {}
                });

                match app.scroll_bar_always_visible {
                    true => {
                        ui.style_mut().spacing.scroll = egui::style::ScrollStyle::solid();
                        app.style_override.scroll_bar_visibility = ScrollBarVisibility::AlwaysVisible;
                    },
                    false => {
                        ui.style_mut().spacing.scroll = egui::style::ScrollStyle::floating();
                        app.style_override.scroll_bar_visibility = ScrollBarVisibility::VisibleWhenNeeded;
                    }
                };

                //let available = ui.available_size();

                ui.add(
                    //available,
                    egui_data_table::Renderer::new(&mut app.table, &mut app.viewer).with_style(app.style_override),
                );

                // Sync renaming state back from viewer
                app.renaming_item = app.viewer.renaming_item.clone();

                self.handle_viewer_requests(app);
            });
        });
    }

    pub fn ui_row_context_menu(viewer: &mut crate::view::Viewer, ui: &mut egui::Ui, column: usize) {
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
        viewer: &mut crate::view::Viewer,
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

    fn handle_viewer_requests(&mut self, app: &mut CorrelateApp) {
        // Handle row renaming request
        if let Some(row_idx) = app.viewer.rename_row_requested.take() {
            let name_col_idx = app.viewer.column_configs.iter().position(|c| c.is_name)
                .or_else(|| app.viewer.column_configs.iter().position(|c| c.name.contains("Name")))
                .or_else(|| app.viewer.column_configs.iter().position(|c| c.column_type == crate::data::ColumnType::String))
                .unwrap_or(0);
            
            if let Some(row) = app.table.get(row_idx) {
                let current_name = match &row.cells[name_col_idx] {
                    CellValue::String(s) => s.clone(),
                    CellValue::Int(i) => i.to_string(),
                    CellValue::Float(f) => f.to_string(),
                    CellValue::DateTime(dt) => dt.clone(),
                    CellValue::Bool(b) => b.to_string(),
                };
                app.renaming_item = Some((RenamingTarget::Row(row_idx), current_name));
                app.viewer.renaming_item = app.renaming_item.clone();
            }
        }

        // Handle column renaming request
        if let Some(col_idx) = app.viewer.rename_column_requested.take() {
            if let Some(config) = app.viewer.column_configs.get(col_idx) {
                let display_name = config.display_name.as_ref().unwrap_or(&config.name).clone();
                app.renaming_item = Some((RenamingTarget::Column(col_idx), display_name));
                app.viewer.renaming_item = app.renaming_item.clone();
            }
        }

        // Handle row renaming completion
        if app.viewer.rename_committed {
            app.viewer.rename_committed = false;
            if let Some((target, new_name)) = app.renaming_item.take() {
                match target {
                    RenamingTarget::Row(row_idx) => {
                        let name_col_idx = app.viewer.column_configs.iter().position(|c| c.is_name)
                            .or_else(|| app.viewer.column_configs.iter().position(|c| c.name.contains("Name")))
                            .or_else(|| app.viewer.column_configs.iter().position(|c| c.column_type == crate::data::ColumnType::String))
                            .unwrap_or(0);

                        if let Some(row) = app.table.get_mut(row_idx) {
                            row.cells[name_col_idx] = CellValue::String(new_name);
                            app.viewer.save_requested = true;
                        }
                    }
                    RenamingTarget::Column(col_idx) => {
                        if let Some(config) = app.viewer.column_configs.get_mut(col_idx) {
                            config.display_name = if new_name.is_empty() || new_name == config.name { None } else { Some(new_name) };
                            if config.is_virtual {
                                config.name = config.display_name.clone().unwrap();
                            }
                            app.viewer.save_requested = true;
                        }
                    }
                    _ => {}
                }
                app.viewer.renaming_item = None;
            }
        }

        // Handle column reordering from the data table
        if let Some(visual_order) = app.table.visual_column_order() {
            let is_identity = visual_order.iter().enumerate().all(|(i, &c)| i == c);
            if !is_identity {
                // Reorder column_configs in the viewer
                let mut new_configs = Vec::with_capacity(app.viewer.column_configs.len());
                for &idx in &visual_order {
                    new_configs.push(app.viewer.column_configs[idx].clone());
                }
                app.viewer.column_configs = new_configs;

                // Reorder cells in all rows
                let mut rows = app.table.take();
                for row in &mut rows {
                    let mut new_cells = Vec::with_capacity(row.cells.len());
                    for &idx in &visual_order {
                        new_cells.push(row.cells[idx].clone());
                    }
                    row.cells = new_cells;
                }
                app.table.replace(rows);
                // Reset visual order in the library to identity
                app.table.reset_visual_column_order();
                // Mark as needing save
                app.viewer.save_requested = true;
            }
        }

        let column_count_changed = app.table.is_empty() || RowViewer::num_columns(&mut app.viewer) != app.table[0].cells.len();
        if app.viewer.add_column_requested.is_some() || app.viewer.save_requested || column_count_changed {
            let add_column_at = app.viewer.add_column_requested.take();
            app.viewer.save_requested = false;
            
            if let Some(at) = add_column_at {
                let new_column = crate::data::ColumnConfig {
                    name: format!("New Column {}", app.viewer.column_configs.len() + 1),
                    display_name: None,
                    column_type: crate::data::ColumnType::String,
                    is_sortable: true,
                    is_key: false,
                    is_name: false,
                    is_virtual: true,
                    order: app.viewer.column_configs.len(),
                    width: None,
                };
                app.viewer.column_configs.insert(at + 1, new_column);
                // Update all rows in the table
                let mut rows = app.table.take();
                for row in &mut rows {
                    row.cells.insert(at + 1, crate::data::CellValue::String("".to_string()));
                }
                app.table.replace(rows);
            } else if column_count_changed {
                // Update all rows in the table if needed (e.g. loading from file with virtual columns)
                let mut rows = app.table.take();
                for row in &mut rows {
                    while row.cells.len() < app.viewer.column_configs.len() {
                        row.cells.push(crate::data::CellValue::String("".to_string()));
                    }
                }
                app.table.replace(rows);
            }

            // Save state back to DataSource
            if let Some(idx) = app.selected_index {
                let ds = &mut app.data_sources[idx];
                let sheet = &mut ds.sheets[ds.selected_sheet_index];
                sheet.column_configs = app.viewer.column_configs.clone();
                for (i, config) in sheet.column_configs.iter_mut().enumerate() {
                    config.order = i;
                }
                sheet.table = app.table.clone();

                app.save_source_config(idx);
            }
        }
    }
}
