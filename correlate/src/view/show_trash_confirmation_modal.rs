use egui::Context;
use std::any::Any;
use crate::application_command::{ApplicationCommand, ApplicationCommandHandler};
use crate::egui_data_table::viewer::RowViewer;
use crate::view::RootViewModel;

pub struct ShowTrashConfirmationModal {
    pub ctx: Context,
}

impl ApplicationCommand for ShowTrashConfirmationModal {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct ShowTrashConfirmationModalHandler;

impl ApplicationCommandHandler for ShowTrashConfirmationModalHandler {
    fn handle(&self, cmd: &dyn Any) {
        if let Some(command) = cmd.downcast_ref::<ShowTrashConfirmationModal>() {
            let ctx = &command.ctx;

            let root_view_model_ptr = ctx.data(|d| d.get_temp::<usize>(egui::Id::new("root_view_model"))).unwrap();
            let view_model = unsafe { &mut *(root_view_model_ptr as *mut RootViewModel) };

            let trash_column_index_id = egui::Id::new("trash_column_index");
            let trash_column_index = ctx.data(|d| d.get_temp::<Option<usize>>(trash_column_index_id)).flatten();
            if let Some(column_idx) = trash_column_index {
                egui::Modal::new(egui::Id::new("confirm_trash_modal")).show(ctx, |ui| {
                    ui.set_width(300.0);
                    ui.heading("Confirm Trash");
                    ui.label(format!("Are you sure you want to delete the column '{}'?", 
                        view_model.viewer.column_configs[column_idx].display_name.as_ref().unwrap_or(&view_model.viewer.column_configs[column_idx].name)));
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        if ui.button("Yes, Delete").clicked() {
                            view_model.viewer.on_column_removed(&mut view_model.table, column_idx);
                            ctx.data_mut(|d| d.insert_temp(trash_column_index_id, None::<usize>));
                        }
                        if ui.button("Cancel").clicked() {
                            ctx.data_mut(|d| d.insert_temp(trash_column_index_id, None::<usize>));
                        }
                    });
                    if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                        ctx.data_mut(|d| d.insert_temp(trash_column_index_id, None::<usize>));
                    }
                });
            }

            let trash_project_index_id = egui::Id::new("trash_project_index");
            let trash_project_index = ctx.data(|d| d.get_temp::<Option<usize>>(trash_project_index_id)).flatten();

            if let Some(project_idx) = trash_project_index {
                egui::Modal::new(egui::Id::new("confirm_trash_project_modal")).show(ctx, |ui| {
                    ui.set_width(300.0);
                    ui.heading("Confirm Delete Project");
                    let project_name = view_model.config.projects.as_ref()
                        .and_then(|p| p.get(project_idx))
                        .map(|p| p.name.clone())
                        .unwrap_or_else(|| "Unknown Project".to_string());
                    ui.label(format!("Are you sure you want to delete the project '{}'?", project_name));
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        if ui.button("Yes, Delete").clicked() {
                            view_model.remove_project(project_idx);
                            ctx.data_mut(|d| d.insert_temp(trash_project_index_id, None::<usize>));
                        }
                        if ui.button("Cancel").clicked() {
                            ctx.data_mut(|d| d.insert_temp(trash_project_index_id, None::<usize>));
                        }
                    });
                    if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                        ctx.data_mut(|d| d.insert_temp(trash_project_index_id, None::<usize>));
                    }
                });
            }

            let trash_datasource_index = ctx.data(|d| d.get_temp::<Option<usize>>(egui::Id::new("trash_datasource_index"))).flatten();
            if let Some(ds_idx) = trash_datasource_index {
                egui::Modal::new(egui::Id::new("confirm_trash_datasource_modal")).show(ctx, |ui| {
                    ui.set_width(300.0);
                    ui.heading("Confirm Delete Data Source");

                    let ds_name = view_model.data_sources.get(ds_idx)
                        .map(|ds| ds.name.as_ref().cloned().unwrap_or_else(|| {
                            std::path::Path::new(&ds.path)
                                .file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("Unknown")
                                .to_string()
                        }))
                        .unwrap_or_else(|| "Unknown Data Source".to_string());
                    ui.label(format!("Are you sure you want to delete the data source '{}'?", ds_name));
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        if ui.button("Yes, Delete").clicked() {
                            view_model.remove_data_source(ds_idx);
                            ctx.data_mut(|d| d.insert_temp(egui::Id::new("trash_datasource_index"), None::<usize>));
                        }
                        if ui.button("Cancel").clicked() {
                            ctx.data_mut(|d| d.insert_temp(egui::Id::new("trash_datasource_index"), None::<usize>));
                        }
                    });
                    if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                        ctx.data_mut(|d| d.insert_temp(egui::Id::new("trash_datasource_index"), None::<usize>));
                    }
                });
            }
        }
    }
}
