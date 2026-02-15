use egui_data_table::RowViewer;
use crate::data::{CellValue, Config, RenamingTarget};
use crate::view::RootViewModel;

pub struct CentralPanelViewModel {
}

impl CentralPanelViewModel {
    pub fn default(_: &Config) -> Self {
        Self {
        }
    }

    pub fn handle_viewer_requests(&mut self, view_model: &mut RootViewModel) {

        // Handle row renaming request
        Self::handle_row_rename_request(view_model);

        // Handle column renaming request
        Self::handle_column_rename_request(view_model);

        // Handle row renaming completion
        Self::handle_row_rename_completion(view_model);

        // Handle column reordering from the data table
        Self::handle_column_reordering(view_model);

        let column_count_changed = view_model.table.is_empty() || RowViewer::num_columns(&mut view_model.viewer) != view_model.table[0].cells.len();
        if view_model.viewer.add_column_requested.is_some() || view_model.viewer.save_requested || column_count_changed {
            let add_column_at = view_model.viewer.add_column_requested.take();
            view_model.viewer.save_requested = false;

            if let Some(at) = add_column_at {
                let new_column = crate::data::ColumnConfig {
                    name: format!("New Column {}", view_model.viewer.column_configs.len() + 1),
                    display_name: None,
                    column_type: crate::data::ColumnType::String,
                    is_key: false,
                    is_name: false,
                    is_virtual: true,
                    order: view_model.viewer.column_configs.len(),
                    width: None,
                };
                view_model.viewer.column_configs.insert(at + 1, new_column);
                // Update all rows in the table
                let mut rows = view_model.table.take();
                for row in &mut rows {
                    row.cells.insert(at + 1, crate::data::CellValue::String("".to_string()));
                }
                view_model.table.replace(rows);
            } else if column_count_changed {
                // Update all rows in the table if needed (e.g. loading from file with virtual columns)
                let mut rows = view_model.table.take();
                for row in &mut rows {
                    while row.cells.len() < view_model.viewer.column_configs.len() {
                        row.cells.push(crate::data::CellValue::String("".to_string()));
                    }
                }
                view_model.table.replace(rows);
            }

            // Save state back to DataSource
            Self::save_datasource_configuration(&view_model);
        }
    }

    fn save_datasource_configuration(view_model: &&mut RootViewModel) {
        if let Some(idx) = view_model.selected_index {
            let ds = &mut view_model.data_sources[idx];
            let sheet = &mut ds.sheets[ds.selected_sheet_index];
            sheet.column_configs = view_model.viewer.column_configs.clone();
            for (i, config) in sheet.column_configs.iter_mut().enumerate() {
                config.order = i;
            }
            sheet.table = view_model.table.clone();

            view_model.save_source_config(idx);
        }
    }

    fn handle_column_reordering(view_model: &mut RootViewModel) {
        if let Some(visual_order) = view_model.table.visual_column_order() {
            let is_identity = visual_order.iter().enumerate().all(|(i, &c)| i == c);
            if !is_identity {
                // Reorder column_configs in the viewer
                let mut new_configs = Vec::with_capacity(view_model.viewer.column_configs.len());
                for &idx in &visual_order {
                    new_configs.push(view_model.viewer.column_configs[idx].clone());
                }
                view_model.viewer.column_configs = new_configs;

                // Reorder cells in all rows
                let mut rows = view_model.table.take();
                for row in &mut rows {
                    let mut new_cells = Vec::with_capacity(row.cells.len());
                    for &idx in &visual_order {
                        new_cells.push(row.cells[idx].clone());
                    }
                    row.cells = new_cells;
                }
                view_model.table.replace(rows);
                // Reset visual order in the library to identity
                view_model.table.reset_visual_column_order();
                // Mark as needing save
                view_model.viewer.save_requested = true;
            }
        }
    }

    fn handle_row_rename_request(view_model: &mut RootViewModel) {
        if let Some(row_idx) = view_model.viewer.rename_row_requested.take() {
            let name_col_idx = view_model.viewer.column_configs.iter().position(|c| c.is_name)
                .or_else(|| view_model.viewer.column_configs.iter().position(|c| c.name.contains("Name")))
                .or_else(|| view_model.viewer.column_configs.iter().position(|c| c.column_type == crate::data::ColumnType::String))
                .unwrap_or(0);

            if let Some(row) = view_model.table.get(row_idx) {
                let current_name = match &row.cells[name_col_idx] {
                    CellValue::String(s) => s.clone(),
                    CellValue::Int(i) => i.to_string(),
                    CellValue::Float(f) => f.to_string(),
                    CellValue::DateTime(dt) => dt.clone(),
                    CellValue::Bool(b) => b.to_string(),
                };
                view_model.renaming_item = Some((RenamingTarget::Row(row_idx), current_name));
                view_model.viewer.renaming_item = view_model.renaming_item.clone();
            }
        }
    }

    fn handle_column_rename_request(view_model: &mut RootViewModel) {
        if let Some(col_idx) = view_model.viewer.rename_column_requested.take() {
            if let Some(config) = view_model.viewer.column_configs.get(col_idx) {
                let display_name = config.display_name.as_ref().unwrap_or(&config.name).clone();
                view_model.renaming_item = Some((RenamingTarget::Column(col_idx), display_name));
                view_model.viewer.renaming_item = view_model.renaming_item.clone();
            }
        }
    }

    fn handle_row_rename_completion(view_model: &mut RootViewModel) {
        if view_model.viewer.rename_committed {
            view_model.viewer.rename_committed = false;
            if let Some((target, new_name)) = view_model.renaming_item.take() {
                match target {
                    RenamingTarget::Row(row_idx) => {
                        let name_col_idx = view_model.viewer.column_configs.iter().position(|c| c.is_name)
                            .or_else(|| view_model.viewer.column_configs.iter().position(|c| c.name.contains("Name")))
                            .or_else(|| view_model.viewer.column_configs.iter().position(|c| c.column_type == crate::data::ColumnType::String))
                            .unwrap_or(0);

                        if let Some(row) = view_model.table.get_mut(row_idx) {
                            row.cells[name_col_idx] = CellValue::String(new_name);
                            view_model.viewer.save_requested = true;
                        }
                    }
                    RenamingTarget::Column(col_idx) => {
                        if let Some(config) = view_model.viewer.column_configs.get_mut(col_idx) {
                            config.display_name = if new_name.is_empty() || new_name == config.name { None } else { Some(new_name) };
                            if config.is_virtual {
                                config.name = config.display_name.clone().unwrap();
                            }
                            view_model.viewer.save_requested = true;
                        }
                    }
                    _ => {}
                }
                view_model.viewer.renaming_item = None;
            }
        }
    }
}