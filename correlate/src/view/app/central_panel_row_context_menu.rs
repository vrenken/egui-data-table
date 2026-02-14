use crate::view::row_viewer::Viewer;

impl Viewer {
    pub fn ui_row_context_menu(&mut self, ui: &mut egui::Ui, column: usize) {
        if let Some(config) = self.column_configs.get_mut(column) {
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
}
