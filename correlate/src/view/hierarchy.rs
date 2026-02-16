use egui::{Sense, Widget};
use crate::view::root_view_model::RootViewModel;
use crate::data::RenamingTarget;

#[derive(Default)]
pub struct HierarchyPanel {}

impl HierarchyPanel {
    pub fn ui(&mut self, view_model: &mut RootViewModel, ctx: &egui::Context) -> (Option<usize>, Option<usize>) {
        let mut newly_selected_index = None;
        let mut newly_selected_sheet_index = None;

        egui::SidePanel::left("hierarchy_panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.heading("Project");
                    ui.separator();

                    let header_res = egui::collapsing_header::CollapsingHeader::new(format!("{} Data sources", egui_material_icons::icons::ICON_FOLDER))
                        .default_open(true)
                        .show(ui, |ui| {
                            
                            let renaming_target_id = egui::Id::new("renaming_target");
                            let renaming_target_opt = ui.data(|d| d.get_temp::<RenamingTarget>(renaming_target_id));

                            for index in 0..view_model.data_sources.len() {
                                let ds = &view_model.data_sources[index];
                                let default_file_name = std::path::Path::new(&ds.path)
                                    .file_name()
                                    .and_then(|n| n.to_str())
                                    .unwrap_or("Unknown")
                                    .to_string();
                                
                                let ds_display_name = ds.name.as_ref().unwrap_or(&default_file_name).clone();
                                
                                let extension = std::path::Path::new(&ds.path)
                                    .extension()
                                    .and_then(|e| e.to_str())
                                    .unwrap_or("");
                                
                                let icon = if extension == "csv" {
                                    egui_material_icons::icons::ICON_CSV
                                } else {
                                    egui_material_icons::icons::ICON_TABLE_CHART
                                };

                                if ds.sheets.len() > 1 {
                                    let mut header = egui::collapsing_header::CollapsingHeader::new(format!("{} {}", icon, ds_display_name))
                                        .default_open(true);
                                    
                                    let renaming_this_ds = renaming_target_opt.map_or(false, |t| t == RenamingTarget::DataSource(index));

                                    if renaming_this_ds {
                                        header = egui::collapsing_header::CollapsingHeader::new(format!("{} ", icon));
                                    }

                                    let header_res = header.show(ui, |ui| {
                                            for sheet_idx in 0..view_model.data_sources[index].sheets.len() {
                                                let selected = view_model.selected_index == Some(index) && view_model.data_sources[index].selected_sheet_index == sheet_idx;
                                                let renaming_target_id = egui::Id::new("renaming_target");
                                                let renaming_target_opt = ui.data(|d| d.get_temp::<RenamingTarget>(renaming_target_id));
                                                let renaming_this_sheet = renaming_target_opt.map_or(false, |t| t == RenamingTarget::Sheet(index, sheet_idx));

                                                let sheet_display_name = view_model.data_sources[index].sheets[sheet_idx].display_name.as_ref().unwrap_or(&view_model.data_sources[index].sheets[sheet_idx].name).clone();

                                                if renaming_this_sheet {
                                                    ui.horizontal(|ui| {
                                                        ui.label("  📄 ");
                                                        
                                                        let rename_id = ui.id().with("rename_sheet");
                                                        let mut current_name = ui.data_mut(|d| d.get_temp::<String>(rename_id).unwrap_or(sheet_display_name.clone()));

                                                        let res = ui.text_edit_singleline(&mut current_name);
                                                        if res.lost_focus() || (ui.input(|i| i.key_pressed(egui::Key::Enter))) {
                                                            view_model.apply_rename(RenamingTarget::Sheet(index, sheet_idx), current_name.clone());
                                                            ui.data_mut(|d| {
                                                                d.remove::<RenamingTarget>(renaming_target_id);
                                                                d.remove::<String>(rename_id);
                                                            });
                                                        } else if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                                                            ui.data_mut(|d| {
                                                                d.remove::<RenamingTarget>(renaming_target_id);
                                                                d.remove::<String>(rename_id);
                                                            });
                                                        } else {
                                                            ui.data_mut(|d| d.insert_temp(rename_id, current_name));
                                                        }
                                                        res.request_focus();
                                                    });
                                                } else {
                                                    let res = ui.selectable_label(selected, format!("  📄 {}", sheet_display_name))
                                                        .on_hover_text(&view_model.data_sources[index].path);
                                    
                                                    if res.clicked() {
                                                        if view_model.selected_index != Some(index) || view_model.data_sources[index].selected_sheet_index != sheet_idx {
                                                            newly_selected_index = Some(index);
                                                            newly_selected_sheet_index = Some(sheet_idx);
                                                        }
                                                    }

                                                    if res.double_clicked() {
                                                        ui.data_mut(|d| d.insert_temp(renaming_target_id, RenamingTarget::Sheet(index, sheet_idx)));
                                                    }
                                                }
                                            }
                                        });

                                    if renaming_this_ds {
                                        let mut rect = header_res.header_response.rect;
                                        rect.min.x += 20.0; // Offset for icon
                                        ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
                                            let rename_id = ui.id().with("rename_ds");
                                            let mut current_name = ui.data_mut(|d| d.get_temp::<String>(rename_id).unwrap_or(ds_display_name.clone()));
                                            let res = ui.text_edit_singleline(&mut current_name);
                                            if res.lost_focus() || (ui.input(|i| i.key_pressed(egui::Key::Enter))) {
                                                view_model.apply_rename(RenamingTarget::DataSource(index), current_name.clone());
                                                ui.data_mut(|d| {
                                                    d.remove::<RenamingTarget>(renaming_target_id);
                                                    d.remove::<String>(rename_id);
                                                });
                                            } else if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                                                ui.data_mut(|d| {
                                                    d.remove::<RenamingTarget>(renaming_target_id);
                                                    d.remove::<String>(rename_id);
                                                });
                                            } else {
                                                ui.data_mut(|d| d.insert_temp(rename_id, current_name));
                                            }
                                            res.request_focus();
                                        });
                                    } else {
                                        if header_res.header_response.clicked() {
                                            if view_model.selected_index != Some(index) {
                                                newly_selected_index = Some(index);
                                                newly_selected_sheet_index = Some(view_model.data_sources[index].selected_sheet_index);
                                            }
                                        }
                                        if header_res.header_response.double_clicked() {
                                            ui.data_mut(|d| d.insert_temp(renaming_target_id, RenamingTarget::DataSource(index)));
                                        }
                                    }
                                } else {
                                    let selected = view_model.selected_index == Some(index);
                                    let renaming_target_id = egui::Id::new("renaming_target");
                                    let renaming_target_opt = ui.data(|d| d.get_temp::<RenamingTarget>(renaming_target_id));
                                    let renaming_this_ds = renaming_target_opt.map_or(false, |t| t == RenamingTarget::DataSource(index));

                                    if renaming_this_ds {
                                        ui.horizontal(|ui| {
                                            ui.label(format!("{} ", icon));
                                            let rename_id = ui.id().with("rename_ds");
                                            let mut current_name = ui.data_mut(|d| d.get_temp::<String>(rename_id).unwrap_or(ds_display_name.clone()));
                                            let res = ui.text_edit_singleline(&mut current_name);
                                            if res.lost_focus() || (ui.input(|i| i.key_pressed(egui::Key::Enter))) {
                                                view_model.apply_rename(RenamingTarget::DataSource(index), current_name.clone());
                                                ui.data_mut(|d| {
                                                    d.remove::<RenamingTarget>(renaming_target_id);
                                                    d.remove::<String>(rename_id);
                                                });
                                            } else if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                                                ui.data_mut(|d| {
                                                    d.remove::<RenamingTarget>(renaming_target_id);
                                                    d.remove::<String>(rename_id);
                                                });
                                            } else {
                                                ui.data_mut(|d| d.insert_temp(rename_id, current_name));
                                            }
                                            res.request_focus();
                                        });
                                    } else {
                                        let res = ui.selectable_label(selected, format!("{} {}", icon, ds_display_name))
                                            .on_hover_text(&ds.path);
                                        
                                        if res.clicked() {
                                            if view_model.selected_index != Some(index) {
                                                newly_selected_index = Some(index);
                                                newly_selected_sheet_index = Some(0);
                                            }
                                        }

                                        if res.double_clicked() {
                                            ui.data_mut(|d| d.insert_temp(renaming_target_id, RenamingTarget::DataSource(index)));
                                        }
                                    }
                                }
                            }

                            if view_model.data_sources.is_empty() {
                                ui.label("No files loaded");
                            }
                        });

                    header_res.header_response.context_menu(|ui| {
                        if let Some(path) = Self::ui_hierarchy_panel_context_menu(ui) {
                            view_model.pending_file_to_add = Some(path);
                        }
                    });

                    ui.add_space(20.);

                    ui.heading("Configuration");
                    ui.separator();

                    if ui.button("💾 Save as default").clicked() {
                        let config_path = "config.json";
                        view_model.config.data_sources = view_model.data_sources.iter().map(|ds| ds.path.clone()).collect();
                        view_model.config.selected_index = view_model.selected_index;
                        if let Err(e) = view_model.config.save(config_path) {
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
                        egui::Button::new(format!("{a:?}"))
                            .shortcut_text(ctx.format_shortcut(k))
                            .wrap_mode(egui::TextWrapMode::Wrap)
                            .sense(Sense::hover())
                            .ui(ui);
                    }
                });
            });

        (newly_selected_index, newly_selected_sheet_index)
    }

    pub fn ui_hierarchy_panel_context_menu(ui: &mut egui::Ui) -> Option<std::path::PathBuf> {
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
