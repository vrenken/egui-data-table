use egui::{Context, Id};
use std::any::Any;
use crate::application_command::{ApplicationCommand, ApplicationCommandHandler};
use crate::data::*;
use crate::view::*;
use crate::egui_data_table::DataTable;

pub struct TrashColumn {
    pub ctx: Context,
    pub column: usize,
}

impl ApplicationCommand for TrashColumn {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct TrashColumnHandler;

impl TrashColumnHandler {
    fn on_column_removed(&self, row_view: &mut RowView, table: &mut DataTable<Row>, index: usize) {
        if index >= row_view.column_configs.len() {
            return;
        }

        // Remove column config
        row_view.column_configs.remove(index);

        // Update all rows in the table
        let mut rows = table.take();
        for row in &mut rows {
            if index < row.cells.len() {
                row.cells.remove(index);
            }
        }
        table.replace(rows);
        table.mark_as_modified();
    }
}
impl ApplicationCommandHandler for TrashColumnHandler {
    fn handle(&self, command: &dyn Any) {
        if let Some(command) = command.downcast_ref::<TrashColumn>() {
            let view_model_ptr = command.ctx.data(|d| d.get_temp::<usize>(Id::new("root_view_model"))).unwrap();
            let view_model = unsafe { &mut *(view_model_ptr as *mut RootViewModel) };
            let column_idx = command.column;

            // let ctx = &command.ctx;
            // egui::Modal::new(Id::new("confirm_trash_modal")).show(ctx, |ui| {
            //     ui.set_width(300.0);
            //     ui.heading("Confirm Trash");
            //     ui.label(format!("Are you sure you want to delete the column '{}'?",
            //                      view_model.viewer.column_configs[column_idx].display_name.as_ref().unwrap_or(&view_model.viewer.column_configs[column_idx].name)));
            //     ui.add_space(10.0);
            //     ui.horizontal(|ui| {
            //         if ui.button("Yes, Delete").clicked() {
            //             self.on_column_removed(&mut view_model.viewer, &mut view_model.table, column_idx);
            //         }
            //     });
            // });
            // TODO: Re-add confirmation dialog.
            self.on_column_removed(&mut view_model.viewer, &mut view_model.table, column_idx);
        }
    }
}
