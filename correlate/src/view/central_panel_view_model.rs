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
        // Update visible columns snapshot for viewer (used by column header menu)
        view_model.viewer.visible_columns = view_model.table.visual_column_order();

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
            let total = view_model.viewer.column_configs.len();
            if total == 0 { return; }

            // visual_order only contains visible columns. Build a full permutation that keeps
            // hidden columns (not present in visual_order) at the end preserving their
            // original relative order.
            let visible_set: std::collections::HashSet<usize> = visual_order.iter().cloned().collect();
            let hidden: Vec<usize> = (0..total).filter(|i| !visible_set.contains(i)).collect();
            let mut full_order = visual_order.clone();
            full_order.extend(hidden.iter().cloned());

            // Always update visibility flags based on the visual_order from library
            let mut any_vis_change = false;
            for (i, cfg) in view_model.viewer.column_configs.iter_mut().enumerate() {
                let should_be_visible = visible_set.contains(&i);
                if cfg.is_visible != should_be_visible {
                    any_vis_change = true;
                }
            }

            let is_identity = full_order.iter().enumerate().all(|(i, &c)| i == c);
            if !is_identity {
                // Reorder column_configs in the viewer based on full_order
                let mut new_configs = Vec::with_capacity(total);
                for &idx in &full_order {
                    let cfg = view_model.viewer.column_configs[idx].clone();
                    // No need to set is_visible here as we just updated it above, 
                    // and visible_set is derived from idx being in visual_order which is correctly reflected in full_order.
                    new_configs.push(cfg);
                }
                view_model.viewer.column_configs = new_configs;

                // Update all rows in the table if needed (e.g. loading from file with virtual columns)
                let mut rows = view_model.table.take();
                for row in &mut rows {
                    let mut new_cells = Vec::with_capacity(row.cells.len());
                    for &idx in &full_order {
                        new_cells.push(row.cells[idx].clone());
                    }
                    row.cells = new_cells;
                }
                view_model.table.replace(rows);

                // Re-calculate visible indices after physical reordering
                let new_visible_indices: Vec<usize> = (0..visual_order.len()).collect();
                view_model.table.set_visual_column_order(new_visible_indices);

                // Persist configuration changes (visibility and order)
                view_model.save_datasource_configuration();
            } else if any_vis_change {
                // If order is identity but visibility changed, still persist
                view_model.save_datasource_configuration();
            }
        } else {
            // If library doesn't have a visual order yet (first frame), 
            // initialize it from our column_configs' is_visible flags.
            let visible_indices: Vec<usize> = view_model.viewer.column_configs.iter()
                .enumerate()
                .filter(|(_, cfg)| cfg.is_visible)
                .map(|(i, _)| i)
                .collect();
            
            if !visible_indices.is_empty() && visible_indices.len() < view_model.viewer.column_configs.len() {
                view_model.table.set_visual_column_order(visible_indices);
            }
        }
    }


}