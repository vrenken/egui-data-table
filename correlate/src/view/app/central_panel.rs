use egui::scroll_area::ScrollBarVisibility;
use egui_data_table::RowViewer;
use crate::view::CorrelateApp;

impl CorrelateApp {
    pub fn ui_central_panel(&mut self, ctx: &egui::Context) {
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
