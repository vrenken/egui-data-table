use egui::*;
use crate::view::*;
use crate::data::*;

#[derive(Default)]
pub struct HierarchyPanel {}

impl HierarchyPanel {
    pub fn ui(&mut self, view_model: &mut RootViewModel, ctx: &Context) -> (Option<usize>, Option<usize>) {
        let mut newly_selected_index = None;
        let mut newly_selected_sheet_index = None;

        SidePanel::left("hierarchy_panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.heading("Project");
                    ui.separator();

                    let header_res = CollapsingHeader::new(format!("{} Projects", egui_material_icons::icons::ICON_FOLDER))
                        .default_open(true)
                        .show(ui, |ui| {
                            
                            let renaming_target_id = Id::new("renaming_target");
                            let renaming_target_opt = ui.data(|d| d.get_temp::<Rename>(renaming_target_id));

                            if let Some(project_configs) = view_model.config.projects.clone() {
                                for project_idx in 0..project_configs.len() {
                                    let project = Project {
                                        configuration: project_configs[project_idx].clone(),
                                    };
                                    project.ui(ui, project_idx, renaming_target_opt, view_model);
                                }
                            }

                            ui.add_space(10.0);
                            ui.label("Data sources:");

                            for index in 0..view_model.data_sources.len() {
                                let ds = &view_model.data_sources[index];
                                let default_file_name = std::path::Path::new(&ds.path)
                                    .file_name()
                                    .and_then(|n| n.to_str())
                                    .unwrap_or("Unknown")
                                    .to_string();
                                
                                let ds_display_name = ds.name.as_ref().unwrap_or(&default_file_name).clone();
                                
                                let icon = ds.sheets.first().map(|s| s.icon).unwrap_or(egui_material_icons::icons::ICON_TABLE_CHART);

                                if ds.sheets.len() > 1 {
                                    let mut header = CollapsingHeader::new(format!("{} {}", icon, ds_display_name))
                                        .default_open(true);

                                    let renaming_this_ds = renaming_target_opt.map_or(false, |t| t == Rename::DataSource(index));

                                    if renaming_this_ds {
                                        header = CollapsingHeader::new(format!("{} ", icon));
                                    }

                                    let header_res = header.show(ui, |ui| {
                                            for sheet_idx in 0..view_model.data_sources[index].sheets.len() {
                                                let selected = view_model.selected_index == Some(index) && view_model.data_sources[index].selected_sheet_index == sheet_idx;
                                                let renaming_target_id = Id::new("renaming_target");
                                                let renaming_target_opt = ui.data(|d| d.get_temp::<Rename>(renaming_target_id));
                                                let renaming_this_sheet = renaming_target_opt.map_or(false, |t| t == Rename::Sheet(index, sheet_idx));

                                                let sheet_display_name = view_model.data_sources[index].sheets[sheet_idx].display_name.as_ref().unwrap_or(&view_model.data_sources[index].sheets[sheet_idx].name).clone();

                                                if renaming_this_sheet {
                                                    Project::ui_item_as_editable(ui, view_model, Rename::Sheet(index, sheet_idx), ui.id().with("rename_sheet"), "  📄", &sheet_display_name);
                                                } else {
                                                    Project::ui_item_as_selectable(ui, Rename::Sheet(index, sheet_idx), selected, "  📄", &sheet_display_name, Some(&view_model.data_sources[index].path), || {
                                                        if view_model.selected_index != Some(index) || view_model.data_sources[index].selected_sheet_index != sheet_idx {
                                                            newly_selected_index = Some(index);
                                                            newly_selected_sheet_index = Some(sheet_idx);
                                                        }
                                                    });
                                                }
                                            }
                                        });

                                    if renaming_this_ds {
                                        let mut rect = header_res.header_response.rect;
                                        rect.min.x += 20.0; // Offset for icon
                                        header_res.header_response.context_menu(|ui| {
                                            Project::ui_item_context_menu(ui, Rename::DataSource(index));
                                        });
                                        ui.scope_builder(UiBuilder::new().max_rect(rect), |ui| {
                                            Project::ui_item_as_editable(ui, view_model, Rename::DataSource(index), ui.id().with("rename_ds"), "", &ds_display_name);
                                        });
                                    } else {
                                        header_res.header_response.context_menu(|ui| {
                                            Project::ui_item_context_menu(ui, Rename::DataSource(index));
                                        });

                                        if header_res.header_response.clicked() {
                                            if view_model.selected_index != Some(index) {
                                                newly_selected_index = Some(index);
                                                newly_selected_sheet_index = Some(view_model.data_sources[index].selected_sheet_index);
                                            }
                                        }
                                        if header_res.header_response.double_clicked() {
                                            ui.data_mut(|d| d.insert_temp(renaming_target_id, Rename::DataSource(index)));
                                        }
                                    }
                                } else {
                                    let selected = view_model.selected_index == Some(index);
                                    let renaming_target_id = Id::new("renaming_target");
                                    let renaming_target_opt = ui.data(|d| d.get_temp::<Rename>(renaming_target_id));
                                    let renaming_this_ds = renaming_target_opt.map_or(false, |t| t == Rename::DataSource(index));

                                    if renaming_this_ds {
                                        Project::ui_item_as_editable(ui, view_model, Rename::DataSource(index), ui.id().with("rename_ds"), icon, &ds_display_name);
                                    } else {
                                        Project::ui_item_as_selectable(ui, Rename::DataSource(index), selected, icon, &ds_display_name, Some(&ds.path), || {
                                            if view_model.selected_index != Some(index) {
                                                newly_selected_index = Some(index);
                                                newly_selected_sheet_index = Some(0);
                                            }
                                        });
                                    }
                                }
                            }

                            if view_model.data_sources.is_empty() {
                                ui.label("No files loaded");
                            }
                        });

                    header_res.header_response.context_menu(|ui| {
                        if ui.button("Add project").clicked() {
                            view_model.add_project();
                            ui.close();
                        }
                        ui.separator();
                        if let Some(path) = Self::ui_hierarchy_panel_context_menu(ui) {
                            view_model.pending_file_to_add = Some(path);
                        }
                    });

                    ui.add_space(20.);

                    ui.heading("Configuration");
                    ui.separator();

                    if ui.button("💾 Save as default").clicked() {
                        view_model.config.data_sources = view_model.data_sources.iter().map(|ds| ds.path.clone()).collect();
                        view_model.config.selected_index = view_model.selected_index;
                        if let Err(e) = view_model.config.save() {
                            log::error!("Failed to save config: {}", e);
                        }
                        // Also save .correlate files for all data sources
                        for i in 0..view_model.data_sources.len() {
                            view_model.save_source_config(i);
                        }
                    }

                    ui.add_space(20.);

                    ui.heading("Hotkeys");
                    ui.separator();
                    ui.add_space(0.);
                    for (k, a) in &view_model.viewer.hotkeys {
                        Button::new(format!("{a:?}"))
                            .shortcut_text(ctx.format_shortcut(k))
                            .wrap_mode(TextWrapMode::Wrap)
                            .sense(Sense::hover())
                            .ui(ui);
                    }
                });
            });

        (newly_selected_index, newly_selected_sheet_index)
    }

    pub fn ui_hierarchy_panel_context_menu(ui: &mut Ui) -> Option<std::path::PathBuf> {
        let mut result = None;
        if ui.button("Add data source").clicked() {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("Excel Files", &["xlsx"])
                .add_filter("CSV Files", &["csv"])
                .pick_file() 
            {
                result = Some(path);
            }
            ui.close();
        }
        result
    }
}
