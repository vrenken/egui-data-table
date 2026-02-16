use egui_data_table::RowViewer;
use crate::data::Config;
use crate::view::RootViewModel;

pub struct CentralPanelViewModel {
}

impl CentralPanelViewModel {
    pub fn default(_: &Config) -> Self {
        Self {
        }
    }

    pub fn handle_viewer_requests(&mut self, view_model: &mut RootViewModel) {

        // Handle column reordering from the data table
        Self::handle_column_reordering(view_model);

        let column_count_changed = view_model.table.is_empty() || RowViewer::num_columns(&mut view_model.viewer) != view_model.table[0].cells.len();
        if column_count_changed {
            // Update all rows in the table if needed (e.g. loading from file with virtual columns)
            let mut rows = view_model.table.take();
            for row in &mut rows {
                while row.cells.len() < view_model.viewer.column_configs.len() {
                    let next_col_idx = row.cells.len();
                    row.cells.push(view_model.viewer.column_configs[next_col_idx].column_type.default_value());
                }
            }
            view_model.table.replace(rows);

            // Save state back to DataSource
            view_model.save_datasource_configuration();
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
                view_model.save_datasource_configuration();
            }
        }
    }


}