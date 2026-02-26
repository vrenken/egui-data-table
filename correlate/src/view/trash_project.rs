use egui::{Context, Id};
use std::any::Any;
use crate::application_command::{ApplicationCommand, ApplicationCommandHandler};
use crate::view::*;

pub struct TrashProject {
    pub ctx: Context,
    pub project: usize,
}

impl ApplicationCommand for TrashProject {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct TrashProjectHandler;

impl TrashProjectHandler {
    pub fn remove_project(&self, view_model: &mut RootViewModel, index: usize) {
        if let Some(projects) = view_model.config.projects.as_mut() {
            if index < projects.len() {
                projects.remove(index);
                if let Err(e) = view_model.config.save() {
                    log::error!("Failed to save config after removing project: {}", e);
                }
            }
        }
    }
}

impl ApplicationCommandHandler for TrashProjectHandler {
    fn handle(&self, command: &dyn Any) {
        if let Some(command) = command.downcast_ref::<TrashProject>() {
            let view_model_ptr = command.ctx.data(|d| d.get_temp::<usize>(Id::new("root_view_model"))).unwrap();
            let view_model = unsafe { &mut *(view_model_ptr as *mut RootViewModel) };
            let project_idx = command.project;

            let ctx = &command.ctx;
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
                        self.remove_project(view_model, project_idx);
                        //ctx.data_mut(|d| d.insert_temp(trash_project_index_id, None::<usize>));
                    }
                    if ui.button("Cancel").clicked() {
                        //ctx.data_mut(|d| d.insert_temp(trash_project_index_id, None::<usize>));
                    }
                });
                // if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                //     ctx.data_mut(|d| d.insert_temp(trash_project_index_id, None::<usize>));
                // }
            });
            self.remove_project(view_model, project_idx);
        }
    }
}
