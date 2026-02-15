use egui::{Sense, Widget};
use crate::view::root_view_model::RootViewModel;
use crate::data::RenamingTarget;

#[derive(Default)]
pub struct HierarchyPanel {}

impl HierarchyPanel {
    pub fn ui(&mut self, view_model: &mut RootViewModel, ctx: &egui::Context) -> (Option<usize>, Option<usize>) {
        let mut newly_selected_index = None;
        let mut newly_selected_sheet_index = None;

        egui::SidePanel::left("left_panel")
            .default_width(250.)
            .show(ctx, |ui| {
                ui.vertical_centered_justified(|ui| {
                    egui::ScrollArea::vertical()
                        .id_salt("hierarchy_scroll")
                        .show(ui, |ui| {
                            ui.set_min_width(ui.available_width());
                            let header_response = egui::collapsing_header::CollapsingHeader::new(egui::RichText::new(format!("{} Data Sources", egui_material_icons::icons::ICON_HOME_STORAGE)).strong())
                                .open(Some(true))
                                .show(ui, |ui| {
                                    for (index, ds) in view_model.data_sources.iter_mut().enumerate() {
                                        let default_file_name = std::path::Path::new(&ds.path)
                                            .file_name()
                                            .and_then(|n| n.to_str())
                                            .unwrap_or(&ds.path)
                                            .to_string();
                                        
                                        let display_name = ds.name.as_ref().unwrap_or(&default_file_name).clone();
                                        
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
                                            let mut header = egui::collapsing_header::CollapsingHeader::new(format!("{} {}", icon, display_name))
                                                .default_open(true);
                                            
                                            let renaming_this_ds = view_model.renaming_item.as_ref().map_or(false, |(t, _)| *t == RenamingTarget::DataSource(index));

                                            if renaming_this_ds {
                                                header = egui::collapsing_header::CollapsingHeader::new(format!("{} ", icon));
                                            }

                                            let header_res = header.show(ui, |ui| {
                                                    for (sheet_idx, sheet) in ds.sheets.iter_mut().enumerate() {
                                                        let selected = view_model.selected_index == Some(index) && ds.selected_sheet_index == sheet_idx;
                                                        let renaming_this_sheet = view_model.renaming_item.as_ref().map_or(false, |(t, _)| *t == RenamingTarget::Sheet(index, sheet_idx));

                                                        let display_name = sheet.display_name.as_ref().unwrap_or(&sheet.name).clone();

                                                        if renaming_this_sheet {
                                                            ui.horizontal(|ui| {
                                                                ui.label("  📄 ");
                                                                let (_, current_name) = view_model.renaming_item.as_mut().unwrap();
                                                                let res = ui.text_edit_singleline(current_name);
                                                                if res.lost_focus() || (ui.input(|i| i.key_pressed(egui::Key::Enter))) {
                                                                    sheet.display_name = if current_name.is_empty() || current_name == &sheet.name { None } else { Some(current_name.clone()) };
                                                                    view_model.renaming_item = None;
                                                                    view_model.save_requested = Some(index);
                                                                }
                                                                if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                                                                    view_model.renaming_item = None;
                                                                }
                                                                res.request_focus();
                                                            });
                                                        } else {
                                                            let res = ui.selectable_label(selected, format!("  📄 {}", display_name))
                                                                .on_hover_text(&ds.path);
                                            
                                                            if res.clicked() {
                                                                if view_model.selected_index != Some(index) || ds.selected_sheet_index != sheet_idx {
                                                                    newly_selected_index = Some(index);
                                                                    newly_selected_sheet_index = Some(sheet_idx);
                                                                }
                                                            }

                                                            if res.double_clicked() {
                                                                view_model.renaming_item = Some((RenamingTarget::Sheet(index, sheet_idx), display_name.clone()));
                                                            }
                                                        }
                                                    }
                                                });

                                            if renaming_this_ds {
                                                let mut rect = header_res.header_response.rect;
                                                rect.min.x += 20.0; // Offset for icon
                                                ui.allocate_ui_at_rect(rect, |ui| {
                                                    let (_, current_name) = view_model.renaming_item.as_mut().unwrap();
                                                    let res = ui.text_edit_singleline(current_name);
                                                    if res.lost_focus() || (ui.input(|i| i.key_pressed(egui::Key::Enter))) {
                                                        ds.name = if current_name.is_empty() || current_name == &default_file_name { None } else { Some(current_name.clone()) };
                                                        view_model.renaming_item = None;
                                                        view_model.save_requested = Some(index);
                                                    }
                                                    if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                                                        view_model.renaming_item = None;
                                                    }
                                                    res.request_focus();
                                                });
                                            } else {
                                                if header_res.header_response.clicked() {
                                                    if view_model.selected_index != Some(index) {
                                                        newly_selected_index = Some(index);
                                                        newly_selected_sheet_index = Some(ds.selected_sheet_index);
                                                    }
                                                }
                                                if header_res.header_response.double_clicked() {
                                                    view_model.renaming_item = Some((RenamingTarget::DataSource(index), display_name.clone()));
                                                }
                                            }
                                        } else {
                                            let selected = view_model.selected_index == Some(index);
                                            let renaming_this_ds = view_model.renaming_item.as_ref().map_or(false, |(t, _)| *t == RenamingTarget::DataSource(index));

                                            if renaming_this_ds {
                                                ui.horizontal(|ui| {
                                                    ui.label(format!("{} ", icon));
                                                    let (_, current_name) = view_model.renaming_item.as_mut().unwrap();
                                                    let res = ui.text_edit_singleline(current_name);
                                                    if res.lost_focus() || (ui.input(|i| i.key_pressed(egui::Key::Enter))) {
                                                        ds.name = if current_name.is_empty() || current_name == &default_file_name { None } else { Some(current_name.clone()) };
                                                        // Also sync the single sheet name if they match? 
                                                        // Actually, the user likely wants to rename the displayed name.
                                                        // If it's single sheet, we might want to rename the sheet itself too.
                                                        if let Some(sheet) = ds.sheets.get_mut(0) {
                                                            sheet.display_name = if current_name.is_empty() || current_name == &sheet.name { None } else { Some(current_name.clone()) };
                                                        }
                                                        view_model.renaming_item = None;
                                                        view_model.save_requested = Some(index);
                                                    }
                                                    if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                                                        view_model.renaming_item = None;
                                                    }
                                                    res.request_focus();
                                                });
                                            } else {
                                                let res = ui.selectable_label(selected, format!("{} {}", icon, display_name))
                                                    .on_hover_text(&ds.path);
                                                
                                                if res.clicked() {
                                                    if view_model.selected_index != Some(index) {
                                                        newly_selected_index = Some(index);
                                                        newly_selected_sheet_index = Some(0);
                                                    }
                                                }

                                                if res.double_clicked() {
                                                    view_model.renaming_item = Some((RenamingTarget::DataSource(index), display_name.clone()));
                                                }
                                            }
                                        }
                                    }

                                    if view_model.data_sources.is_empty() {
                                        ui.label("No files loaded");
                                    }
                                });
                            
                            header_response.header_response.context_menu(|ui| {
                                view_model.pending_file_to_add = HierarchyPanel::ui_hierarchy_panel_context_menu(ui);
                            });
                        });

                    ui.add_space(20.);

                    ui.heading("Configuration");
                    ui.separator();

                    if ui.button(" Reload config.json").clicked() {
                        // TODO: This is tricky with ViewModel. For now, maybe just a signal?
                        // For now I'll leave it as it was, but it won't work because it resets the whole app.
                        // *app = CorrelateApp::default();
                    }

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
        if ui.button("Add").clicked() {
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
