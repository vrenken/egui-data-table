use crate::view::*;
use crate::egui_data_table::*;
use crate::egui_data_table::command::Command;
use crate::data::Row;
use eframe::emath::Align;
use egui::Layout;
use egui::scroll_area::ScrollBarVisibility;
use crate::egui_data_table::renderer::Renderer;

#[derive(Default)]
pub struct CentralPanel {}

impl CentralPanel {
    pub fn update(&mut self,
                  view_model: &mut RootViewModel,
                  central_panel_view_model: &mut CentralPanelViewModel,
                  ctx: &egui::Context,
                  commands: Vec<Command<Row>>) {
        central_panel_view_model.handle_viewer_requests(view_model);
        Self::show_trash_confirmation_modal(ctx, view_model);

        for command in commands {
            match command {
                Command::ToggleScrollBarVisibility => {
                    view_model.scroll_bar_always_visible = !view_model.scroll_bar_always_visible;
                    if view_model.scroll_bar_always_visible {
                        view_model.style_override.scroll_bar_visibility = ScrollBarVisibility::AlwaysVisible;
                    } else {
                        view_model.style_override.scroll_bar_visibility = ScrollBarVisibility::VisibleWhenNeeded;
                    }
                }
                Command::ClearUserModificationFlag => {
                    view_model.table.clear_user_modification_flag();
                    view_model.save_datasource_configuration();
                }
                _ => {}
            }
        }
    }

    pub fn ui(&mut self,
              view_model: &mut RootViewModel,
              ctx: &egui::Context) -> Vec<Command<Row>> {

        let mut commands = Vec::new();

        ctx.data_mut(|d| d.insert_temp(egui::Id::new("root_view_model"), view_model as *mut RootViewModel as usize));

        egui::CentralPanel::default()
            .show(ctx, |ui| {

            ui.vertical(|ui| {

                ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                    if ui.button(egui_material_icons::icons::ICON_PAGE_INFO).clicked() {}
                    if ui.button(egui_material_icons::icons::ICON_SWAP_VERT).clicked() {}
                    if ui.button(egui_material_icons::icons::ICON_FILTER_LIST).clicked() {
                        commands.push(Command::ToggleScrollBarVisibility);
                    }
                });

                match view_model.scroll_bar_always_visible {
                    true => {
                        ui.style_mut().spacing.scroll = egui::style::ScrollStyle::solid();
                    },
                    false => {
                        ui.style_mut().spacing.scroll = egui::style::ScrollStyle::floating();
                    }
                };

                //let available = ui.available_size();

                ui.add(
                    //available,
                    Renderer::new(&mut view_model.table, &mut view_model.viewer).with_style(view_model.style_override),
                );

                if view_model.table.has_user_modification() {
                    commands.push(Command::ClearUserModificationFlag);
                }
            });
        });

        commands
    }

    fn show_trash_confirmation_modal(ctx: &egui::Context, view_model: &mut RootViewModel) {
        let trash_column_index = ctx.data(|d| d.get_temp::<Option<usize>>(egui::Id::new("trash_column_index"))).flatten();
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
                        ctx.data_mut(|d| d.insert_temp(egui::Id::new("trash_column_index"), None::<usize>));
                    }
                    if ui.button("Cancel").clicked() {
                        ctx.data_mut(|d| d.insert_temp(egui::Id::new("trash_column_index"), None::<usize>));
                    }
                });
                if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                    ctx.data_mut(|d| d.insert_temp(egui::Id::new("trash_column_index"), None::<usize>));
                }
            });
        }

        let trash_project_index = ctx.data(|d| d.get_temp::<Option<usize>>(egui::Id::new("trash_project_index"))).flatten();
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
                        ctx.data_mut(|d| d.insert_temp(egui::Id::new("trash_project_index"), None::<usize>));
                    }
                    if ui.button("Cancel").clicked() {
                        ctx.data_mut(|d| d.insert_temp(egui::Id::new("trash_project_index"), None::<usize>));
                    }
                });
                if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                    ctx.data_mut(|d| d.insert_temp(egui::Id::new("trash_project_index"), None::<usize>));
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

    pub fn ui_row_context_menu(viewer: &mut RowView, ui: &mut egui::Ui, column: usize) {
        if let Some(config) = viewer.column_configs.get_mut(column) {
            let mut is_key = config.is_key;
            if ui.checkbox(&mut is_key, "Is key").clicked() {
                config.is_key = is_key;
                // Reset the table to force redrawing with new header names
                ui.ctx().memory_mut(|_mem| {
                    // This is a hacky way to force a full redrawing of the table
                    // by clearing its UI state cache if we had access to the ID.
                    // Since we don't easily have the ID here, we just hope the change
                    // is picked up on the next frame.
                });
                ui.close();
            }
        }
    }
}
