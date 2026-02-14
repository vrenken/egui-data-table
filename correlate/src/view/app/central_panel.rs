use egui::scroll_area::ScrollBarVisibility;
use egui_data_table::RowViewer;
use crate::view::CorrelateApp;
use crate::view::app::types::RenamingTarget;
use crate::data::CellValue;

impl CorrelateApp {
    pub fn ui_central_panel(&mut self, ctx: &egui::Context) {
        // Sync renaming state to viewer
        self.viewer.renaming_item = self.renaming_item.clone();

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.scroll_bar_always_visible {
                true => {
                    ui.style_mut().spacing.scroll = egui::style::ScrollStyle::solid();
                    self.style_override.scroll_bar_visibility = ScrollBarVisibility::AlwaysVisible;
                },
                false => {
                    ui.style_mut().spacing.scroll = egui::style::ScrollStyle::floating();
                    self.style_override.scroll_bar_visibility = ScrollBarVisibility::VisibleWhenNeeded;
                }
            };

            ui.add(
                egui_data_table::Renderer::new(&mut self.table, &mut self.viewer)
                    .with_style(self.style_override),
            );

            // Sync renaming state back from viewer
            self.renaming_item = self.viewer.renaming_item.clone();

            // Handle row renaming request
            if let Some(row_idx) = self.viewer.rename_row_requested.take() {
                let name_col_idx = self.viewer.column_configs.iter().position(|c| c.is_name)
                    .or_else(|| self.viewer.column_configs.iter().position(|c| c.name.contains("Name")))
                    .or_else(|| self.viewer.column_configs.iter().position(|c| c.column_type == crate::data::ColumnType::String))
                    .unwrap_or(0);
                
                if let Some(row) = self.table.get(row_idx) {
                    let current_name = match &row.cells[name_col_idx] {
                        CellValue::String(s) => s.clone(),
                        CellValue::Int(i) => i.to_string(),
                        CellValue::Bool(b) => b.to_string(),
                    };
                    self.renaming_item = Some((RenamingTarget::Row(row_idx), current_name));
                    self.viewer.renaming_item = self.renaming_item.clone();
                }
            }

            // Handle column renaming request
            if let Some(col_idx) = self.viewer.rename_column_requested.take() {
                if let Some(config) = self.viewer.column_configs.get(col_idx) {
                    let display_name = config.display_name.as_ref().unwrap_or(&config.name).clone();
                    self.renaming_item = Some((RenamingTarget::Column(col_idx), display_name));
                    self.viewer.renaming_item = self.renaming_item.clone();
                }
            }

            // Handle row renaming completion
            if self.viewer.rename_committed {
                self.viewer.rename_committed = false;
                if let Some((target, new_name)) = self.renaming_item.take() {
                    match target {
                        RenamingTarget::Row(row_idx) => {
                            let name_col_idx = self.viewer.column_configs.iter().position(|c| c.is_name)
                                .or_else(|| self.viewer.column_configs.iter().position(|c| c.name.contains("Name")))
                                .or_else(|| self.viewer.column_configs.iter().position(|c| c.column_type == crate::data::ColumnType::String))
                                .unwrap_or(0);

                            if let Some(row) = self.table.get_mut(row_idx) {
                                row.cells[name_col_idx] = CellValue::String(new_name);
                                self.viewer.save_requested = true;
                            }
                        }
                        RenamingTarget::Column(col_idx) => {
                            if let Some(config) = self.viewer.column_configs.get_mut(col_idx) {
                                config.display_name = if new_name.is_empty() || new_name == config.name { None } else { Some(new_name) };
                                if config.is_virtual {
                                    config.name = config.display_name.clone().unwrap();
                                }
                                self.viewer.save_requested = true;
                            }
                        }
                        _ => {}
                    }
                    self.viewer.renaming_item = None;
                }
            }

            // Handle column reordering from the data table
            if let Some(visual_order) = self.table.visual_column_order() {
                let is_identity = visual_order.iter().enumerate().all(|(i, &c)| i == c);
                if !is_identity {
                    // Reorder column_configs in the viewer
                    let mut new_configs = Vec::with_capacity(self.viewer.column_configs.len());
                    for &idx in &visual_order {
                        new_configs.push(self.viewer.column_configs[idx].clone());
                    }
                    self.viewer.column_configs = new_configs;

                    // Reorder cells in all rows
                    let mut rows = self.table.take();
                    for row in &mut rows {
                        let mut new_cells = Vec::with_capacity(row.cells.len());
                        for &idx in &visual_order {
                            new_cells.push(row.cells[idx].clone());
                        }
                        row.cells = new_cells;
                    }
                    self.table.replace(rows);
                    // Reset visual order in the library to identity
                    self.table.reset_visual_column_order();
                    // Mark as needing save
                    self.viewer.save_requested = true;
                }
            }

            let column_count_changed = self.table.is_empty() || RowViewer::num_columns(&mut self.viewer) != self.table[0].cells.len();
            if self.viewer.add_column_requested.is_some() || self.viewer.save_requested || column_count_changed {
                let add_column_at = self.viewer.add_column_requested.take();
                self.viewer.save_requested = false;
                
                if let Some(at) = add_column_at {
                    let new_column = crate::data::ColumnConfig {
                        name: format!("New Column {}", self.viewer.column_configs.len() + 1),
                        display_name: None,
                        column_type: crate::data::ColumnType::String,
                        is_sortable: true,
                        is_key: false,
                        is_name: false,
                        is_virtual: true,
                        order: self.viewer.column_configs.len(),
                        width: None,
                    };
                    self.viewer.column_configs.insert(at + 1, new_column);
                    // Update all rows in the table
                    let mut rows = self.table.take();
                    for row in &mut rows {
                        row.cells.insert(at + 1, crate::data::CellValue::String("".to_string()));
                    }
                    self.table.replace(rows);
                } else if column_count_changed {
                    // Update all rows in the table if needed (e.g. loading from file with virtual columns)
                    let mut rows = self.table.take();
                    for row in &mut rows {
                        while row.cells.len() < self.viewer.column_configs.len() {
                            row.cells.push(crate::data::CellValue::String("".to_string()));
                        }
                    }
                    self.table.replace(rows);
                }

                // Save state back to DataSource
                if let Some(idx) = self.selected_index {
                    let ds = &mut self.data_sources[idx];
                    let sheet = &mut ds.sheets[ds.selected_sheet_index];
                    sheet.column_configs = self.viewer.column_configs.clone();
                    for (i, config) in sheet.column_configs.iter_mut().enumerate() {
                        config.order = i;
                    }
                    sheet.table = self.table.clone();

                    self.save_source_config(idx);
                }
            }
        });
    }
}
