use egui::{Sense, Widget};
use crate::view::CorrelateApp;
use crate::view::app::types::RenamingTarget;

impl CorrelateApp {
    pub fn ui_hierarchy_panel(&mut self, ctx: &egui::Context) -> (Option<usize>, Option<usize>) {
        let mut newly_selected_index = None;
        let mut newly_selected_sheet_index = None;

        egui::SidePanel::left("left_panel")
            .default_width(250.)
            .show(ctx, |ui| {
                ui.vertical_centered_justified(|ui| {
                    ui.heading("Hierarchy");
                    ui.separator();

                    egui::ScrollArea::vertical()
                        .id_salt("hierarchy_scroll")
                        .show(ui, |ui| {
                            ui.set_min_width(ui.available_width());
                            let header_response = egui::collapsing_header::CollapsingHeader::new(egui::RichText::new("📁 Data Sources").strong())
                                .default_open(true)
                                .show(ui, |ui| {
                                    for (index, ds) in self.data_sources.iter_mut().enumerate() {
                                        let default_file_name = std::path::Path::new(&ds.path)
                                            .file_name()
                                            .and_then(|n| n.to_str())
                                            .unwrap_or(&ds.path)
                                            .to_string();
                                        
                                        let display_name = ds.name.as_ref().unwrap_or(&default_file_name).clone();
                                        
                                        if ds.sheets.len() > 1 {
                                            let mut header = egui::collapsing_header::CollapsingHeader::new(format!(" {}", display_name))
                                                .default_open(true);
                                            
                                            let renaming_this_ds = self.renaming_item.as_ref().map_or(false, |(t, _)| *t == RenamingTarget::DataSource(index));

                                            if renaming_this_ds {
                                                header = egui::collapsing_header::CollapsingHeader::new(" ");
                                            }

                                            let header_res = header.show(ui, |ui| {
                                                    for (sheet_idx, sheet) in ds.sheets.iter_mut().enumerate() {
                                                        let selected = self.selected_index == Some(index) && ds.selected_sheet_index == sheet_idx;
                                                        let renaming_this_sheet = self.renaming_item.as_ref().map_or(false, |(t, _)| *t == RenamingTarget::Sheet(index, sheet_idx));

                                                        let display_name = sheet.display_name.as_ref().unwrap_or(&sheet.name).clone();

                                                        if renaming_this_sheet {
                                                            ui.horizontal(|ui| {
                                                                ui.label("  📄 ");
                                                                let (_, current_name) = self.renaming_item.as_mut().unwrap();
                                                                let res = ui.text_edit_singleline(current_name);
                                                                if res.lost_focus() || (ui.input(|i| i.key_pressed(egui::Key::Enter))) {
                                                                    sheet.display_name = if current_name.is_empty() || current_name == &sheet.name { None } else { Some(current_name.clone()) };
                                                                    self.renaming_item = None;
                                                                    self.save_requested = Some(index);
                                                                }
                                                                if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                                                                    self.renaming_item = None;
                                                                }
                                                                res.request_focus();
                                                            });
                                                        } else {
                                                            let res = ui.selectable_label(selected, format!("  📄 {}", display_name))
                                                                .on_hover_text(&ds.path);
                                            
                                                            if res.clicked() {
                                                                if self.selected_index != Some(index) || ds.selected_sheet_index != sheet_idx {
                                                                    newly_selected_index = Some(index);
                                                                    newly_selected_sheet_index = Some(sheet_idx);
                                                                }
                                                            }

                                                            if res.double_clicked() {
                                                                self.renaming_item = Some((RenamingTarget::Sheet(index, sheet_idx), display_name.clone()));
                                                            }
                                                        }
                                                    }
                                                });

                                            if renaming_this_ds {
                                                let mut rect = header_res.header_response.rect;
                                                rect.min.x += 20.0; // Offset for icon
                                                ui.allocate_ui_at_rect(rect, |ui| {
                                                    let (_, current_name) = self.renaming_item.as_mut().unwrap();
                                                    let res = ui.text_edit_singleline(current_name);
                                                    if res.lost_focus() || (ui.input(|i| i.key_pressed(egui::Key::Enter))) {
                                                        ds.name = if current_name.is_empty() || current_name == &default_file_name { None } else { Some(current_name.clone()) };
                                                        self.renaming_item = None;
                                                        self.save_requested = Some(index);
                                                    }
                                                    if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                                                        self.renaming_item = None;
                                                    }
                                                    res.request_focus();
                                                });
                                            } else {
                                                if header_res.header_response.clicked() {
                                                    if self.selected_index != Some(index) {
                                                        newly_selected_index = Some(index);
                                                        newly_selected_sheet_index = Some(ds.selected_sheet_index);
                                                    }
                                                }
                                                if header_res.header_response.double_clicked() {
                                                    self.renaming_item = Some((RenamingTarget::DataSource(index), display_name.clone()));
                                                }
                                            }
                                        } else {
                                            let selected = self.selected_index == Some(index);
                                            let renaming_this_ds = self.renaming_item.as_ref().map_or(false, |(t, _)| *t == RenamingTarget::DataSource(index));

                                            if renaming_this_ds {
                                                ui.horizontal(|ui| {
                                                    ui.label(" ");
                                                    let (_, current_name) = self.renaming_item.as_mut().unwrap();
                                                    let res = ui.text_edit_singleline(current_name);
                                                    if res.lost_focus() || (ui.input(|i| i.key_pressed(egui::Key::Enter))) {
                                                        ds.name = if current_name.is_empty() || current_name == &default_file_name { None } else { Some(current_name.clone()) };
                                                        // Also sync the single sheet name if they match? 
                                                        // Actually, the user likely wants to rename the displayed name.
                                                        // If it's single sheet, we might want to rename the sheet itself too.
                                                        if let Some(sheet) = ds.sheets.get_mut(0) {
                                                            sheet.display_name = if current_name.is_empty() || current_name == &sheet.name { None } else { Some(current_name.clone()) };
                                                        }
                                                        self.renaming_item = None;
                                                        self.save_requested = Some(index);
                                                    }
                                                    if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                                                        self.renaming_item = None;
                                                    }
                                                    res.request_focus();
                                                });
                                            } else {
                                                let res = ui.selectable_label(selected, format!(" {}", display_name))
                                                    .on_hover_text(&ds.path);
                                                
                                                if res.clicked() {
                                                    if self.selected_index != Some(index) {
                                                        newly_selected_index = Some(index);
                                                        newly_selected_sheet_index = Some(0);
                                                    }
                                                }

                                                if res.double_clicked() {
                                                    self.renaming_item = Some((RenamingTarget::DataSource(index), display_name.clone()));
                                                }
                                            }
                                        }
                                    }

                                    if self.data_sources.is_empty() {
                                        ui.label("No files loaded");
                                    }
                                });
                            
                            header_response.header_response.context_menu(|ui| {
                                if ui.button("Add").clicked() {
                                    if let Some(path) = rfd::FileDialog::new()
                                        .add_filter("Excel Files", &["xlsx"])
                                        .add_filter("CSV Files", &["csv"])
                                        .pick_file() 
                                    {
                                        self.pending_file_to_add = Some(path);
                                    }
                                    ui.close();
                                }
                            });
                        });

                    ui.add_space(20.);

                    ui.heading("Configuration");
                    ui.separator();

                    if ui.button(" Reload config.json").clicked() {
                        *self = Self::default();
                    }

                    if ui.button("💾 Save as default").clicked() {
                        let config_path = "config.json";
                        self.config.data_sources = self.data_sources.iter().map(|ds| ds.path.clone()).collect();
                        self.config.selected_index = self.selected_index;
                        if let Err(e) = self.config.save(config_path) {
                            log::error!("Failed to save config: {}", e);
                        }
                        // Also save .correlate files for all data sources
                        for i in 0..self.data_sources.len() {
                            self.save_source_config(i);
                        }
                    }

                    ui.add_space(20.);

                    ui.heading("Hotkeys");
                    ui.separator();
                    ui.add_space(0.);
                    for (k, a) in &self.viewer.hotkeys {
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
}
