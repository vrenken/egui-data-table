use egui::{Sense, Widget};
use egui::scroll_area::ScrollBarVisibility;
use crate::data::Row;
use crate::view::Viewer;

pub struct DataSource {
    pub path: String,
    pub column_configs: Vec<crate::data::ColumnConfig>,
    pub table: egui_data_table::DataTable<Row>,
}

pub struct CorrelateApp {
    config: crate::data::Config,
    table: egui_data_table::DataTable<Row>,
    viewer: Viewer,
    data_sources: Vec<DataSource>,
    selected_index: Option<usize>,
    style_override: egui_data_table::Style,
    scroll_bar_always_visible: bool,
    pending_file_to_add: Option<std::path::PathBuf>,
}

impl Default for CorrelateApp {

    fn default() -> Self {
        let config_path = "config.json";
        let config = crate::data::Config::load(config_path).unwrap_or_default();

        let mut data_sources = Vec::new();
        for source in &config.data_sources {
            if source == "Students" {
                let configs = crate::data::get_default_column_configs();
                let rows = crate::data::get_rows(1000, &configs);
                data_sources.push(DataSource {
                    path: "Students".to_string(),
                    column_configs: configs,
                    table: rows.into_iter().collect(),
                });
                continue;
            }

            match crate::data::load_xlsx(source) {
                Ok((column_configs, rows)) => {
                    data_sources.push(DataSource {
                        path: source.to_string(),
                        column_configs,
                        table: rows.into_iter().collect(),
                    });
                }
                Err(e) => {
                    log::error!("Failed to load {}: {}", source, e);
                }
            }
        }

        if data_sources.is_empty() {
            let configs = crate::data::get_default_column_configs();
            let rows = crate::data::get_rows(1000, &configs);
            data_sources.push(DataSource {
                path: "Random Data".to_string(),
                column_configs: configs.clone(),
                table: rows.into_iter().collect(),
            });
        }

        let selected_index = config.selected_index.unwrap_or(0).min(data_sources.len() - 1);
        let ds = &data_sources[selected_index];
        let table = ds.table.clone();
        let viewer = Viewer {
            name_filter: String::new(),
            hotkeys: Vec::new(),
            row_protection: false,
            column_configs: ds.column_configs.clone(),
        };

        Self {
            config,
            table,
            viewer,
            data_sources,
            selected_index: Some(selected_index),
            style_override: Default::default(),
            scroll_bar_always_visible: false,
            pending_file_to_add: None,
        }
    }
}

impl eframe::App for CorrelateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut newly_selected_index = None;

        fn is_send<T: Send>(_: &T) {}
        fn is_sync<T: Sync>(_: &T) {}

        is_send(&self.table);
        is_sync(&self.table);

        egui::TopBottomPanel::top("MenuBar").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.hyperlink_to(
                    " kang-sw/egui-data-table",
                    "https://github.com/kang-sw/egui-data-table",
                );

                // later in some ui:
                ui
                    .button(egui_material_icons::icons::ICON_STRATEGY)
                    .clicked();

                ui.hyperlink_to(
                    "(source)",
                    "https://github.com/kang-sw/egui-data-table/blob/master/examples/demo.rs",
                );

                ui.separator();

                egui::widgets::global_theme_preference_buttons(ui);

                ui.separator();

                ui.label("Name Filter");
                ui.text_edit_singleline(&mut self.viewer.name_filter);

                ui.add(egui::Button::new("Drag me and drop on any cell").sense(Sense::drag()))
                    .on_hover_text(
                        "Dropping this will replace the cell \
                        content with some predefined value.",
                    )
                    .dnd_set_drag_payload(String::from("Hallo~"));

                ui.menu_button("🎌 Flags", |ui| {
                    ui.checkbox(&mut self.viewer.row_protection, "Row Protection")
                        .on_hover_text(
                            "If checked, any rows `Is Student` marked \
                        won't be deleted or overwritten by UI actions.",
                        );

                    ui.checkbox(
                        &mut self.style_override.single_click_edit_mode,
                        "Single Click Edit",
                    )
                    .on_hover_text("If checked, cells will be edited with a single click.");

                    ui.checkbox(
                        &mut self.style_override.auto_shrink.x,
                        "Auto-shrink X",
                    );
                    ui.checkbox(
                        &mut self.style_override.auto_shrink.y,
                        "Auto-shrink Y",
                    );

                    ui.checkbox(
                        &mut self.scroll_bar_always_visible,
                        "Scrollbar always visible",
                    );

                    if ui.button("Shuffle Rows").clicked() {
                        fastrand::shuffle(&mut self.table);
                    }
                })
            })
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            egui::Sides::new().show(ui, |_ui| {
            }, |ui|{
                let mut has_modifications = self.table.has_user_modification();
                ui.add_enabled(false, egui::Checkbox::new(&mut has_modifications, "Has modifications"));

                ui.add_enabled_ui(has_modifications, |ui| {
                    if ui.button("Clear").clicked() {
                        self.table.clear_user_modification_flag();
                    }
                });
            });
        });
        
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
                                    for (index, ds) in self.data_sources.iter().enumerate() {
                                        let file_name = std::path::Path::new(&ds.path)
                                            .file_name()
                                            .and_then(|n| n.to_str())
                                            .unwrap_or(&ds.path);
                                        
                                        ui.horizontal(|ui| {
                                            ui.label(""); // Folder/Open icon
                                            let selected = self.selected_index == Some(index);
                                            if ui.selectable_label(selected, file_name)
                                                .on_hover_text(&ds.path)
                                                .clicked() 
                                            {
                                                if self.selected_index != Some(index) {
                                                    newly_selected_index = Some(index);
                                                }
                                            }
                                        });
                                    }

                                    if self.data_sources.is_empty() {
                                        ui.label("No files loaded");
                                    }
                                });
                            
                            header_response.header_response.context_menu(|ui| {
                                if ui.button("Add").clicked() {
                                    if let Some(path) = rfd::FileDialog::new()
                                        .add_filter("Excel Files", &["xlsx"])
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

        if let Some(index) = newly_selected_index {
            // Save current table state back to its source
            if let Some(old_idx) = self.selected_index {
                self.data_sources[old_idx].table = self.table.clone();
            }

            // Switch to new source
            self.selected_index = Some(index);
            let ds = &self.data_sources[index];
            self.table = ds.table.clone();
            self.viewer.column_configs = ds.column_configs.clone();
        }

        if let Some(path) = self.pending_file_to_add.take() {
            let path_str = path.to_string_lossy().to_string();
            match crate::data::load_xlsx(&path_str) {
                Ok((column_configs, rows)) => {
                    let new_index = self.data_sources.len();
                    self.data_sources.push(DataSource {
                        path: path_str.clone(),
                        column_configs,
                        table: rows.into_iter().collect(),
                    });
                    
                    // Save current table state
                    if let Some(old_idx) = self.selected_index {
                        self.data_sources[old_idx].table = self.table.clone();
                    }

                    // Switch to new source
                    self.selected_index = Some(new_index);
                    let ds = &self.data_sources[new_index];
                    self.table = ds.table.clone();
                    self.viewer.column_configs = ds.column_configs.clone();

                    // Persist to config
                    self.config.data_sources = self.data_sources.iter().map(|ds| ds.path.clone()).collect();
                    self.config.selected_index = self.selected_index;
                    let config_path = "config.json";
                    if let Err(e) = self.config.save(config_path) {
                        log::error!("Failed to save config: {}", e);
                    }
                }
                Err(e) => {
                    log::error!("Failed to load {}: {}", path_str, e);
                }
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.scroll_bar_always_visible {
                true => {
                    ui.style_mut().spacing.scroll = egui::style::ScrollStyle::solid();
                    self.style_override.scroll_bar_visibility = ScrollBarVisibility::AlwaysVisible;
                },
                false => {
                    ui.style_mut().spacing.scroll = egui::style::ScrollStyle::floating();
                    self.style_override.scroll_bar_visibility = ScrollBarVisibility::VisibleWhenNeeded;
                }
            };

            ui.add(
                egui_data_table::Renderer::new(&mut self.table, &mut self.viewer)
                    .with_style(self.style_override),
            )
        });
    }
}