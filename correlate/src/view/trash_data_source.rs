use egui::{Context, Id};
use std::any::Any;
use crate::application_command::*;
use crate::egui_data_table::DataTable;
use crate::view::*;

pub struct TrashDataSource {
    pub ctx: Context,
    pub data_source: usize,
}

impl ApplicationCommand for TrashDataSource {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct TrashDataSourceHandler;

impl TrashDataSourceHandler {
    pub fn remove_data_source(&self, view_model: &mut RootViewModel, index: usize) {
        if index < view_model.data_sources.len() {
            let path_to_remove = view_model.data_sources[index].path.clone();
            view_model.data_sources.remove(index);

            // Update the selected index if necessary
            if let Some(selected) = view_model.selected_index {
                if selected == index {
                    // If we removed the selected one, pick a new one or set to None
                    if view_model.data_sources.is_empty() {
                        view_model.selected_index = None;
                        view_model.table = DataTable::new();
                        view_model.viewer.column_configs = Vec::new();
                        view_model.viewer.data_sources = Vec::new();
                    } else {
                        let new_idx = index.min(view_model.data_sources.len() - 1);
                        view_model.switch_to_source(new_idx, view_model.data_sources[new_idx].selected_sheet_index);
                    }
                } else if selected > index {
                    view_model.selected_index = Some(selected - 1);
                }
            }

            // Also remove from any project that might contain it
            if let Some(projects) = view_model.config.projects.as_mut() {
                for project in projects {
                    project.data_sources.retain(|p| p != &path_to_remove);
                }
            }

            if let Err(e) = view_model.config.save() {
                log::error!("Failed to save config after removing data source: {}", e);
            }
        }
    }
}

impl ApplicationCommandHandler for TrashDataSourceHandler {
    fn handle(&self, command: &dyn Any) {
        if let Some(command) = command.downcast_ref::<TrashDataSource>() {
            let view_model_ptr = command.ctx.data(|d| d.get_temp::<usize>(Id::new("root_view_model"))).unwrap();
            let view_model = unsafe { &mut *(view_model_ptr as *mut RootViewModel) };
            let data_source_idx = command.data_source;

            // let ctx = &command.ctx;
            // egui::Modal::new(egui::Id::new("confirm_trash_datasource_modal")).show(ctx, |ui| {
            //     ui.set_width(300.0);
            //     ui.heading("Confirm Delete Data Source");
            //
            //     let ds_name = view_model.data_sources.get(data_source_idx)
            //         .map(|ds| ds.name.as_ref().cloned().unwrap_or_else(|| {
            //             std::path::Path::new(&ds.path)
            //                 .file_name()
            //                 .and_then(|n| n.to_str())
            //                 .unwrap_or("Unknown")
            //                 .to_string()
            //         }))
            //         .unwrap_or_else(|| "Unknown Data Source".to_string());
            //     ui.label(format!("Are you sure you want to delete the data source '{}'?", ds_name));
            //     ui.add_space(10.0);
            //     ui.horizontal(|ui| {
            //         if ui.button("Yes, Delete").clicked() {
            //             self.remove_data_source(view_model, data_source_idx);
            //             ctx.data_mut(|d| d.insert_temp(egui::Id::new("trash_datasource_index"), None::<usize>));
            //         }
            //         if ui.button("Cancel").clicked() {
            //             ctx.data_mut(|d| d.insert_temp(egui::Id::new("trash_datasource_index"), None::<usize>));
            //         }
            //     });
            //     if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
            //         ctx.data_mut(|d| d.insert_temp(egui::Id::new("trash_datasource_index"), None::<usize>));
            //     }
            // });

            self.remove_data_source(view_model, data_source_idx);
        }
    }
}
