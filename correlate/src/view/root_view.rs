use crate::data::Row;
use crate::view::app::central_panel::CentralPanel;
use crate::view::Viewer;
use crate::view::app::types::{DataSource, RenamingTarget, DataSheet};

pub struct CorrelateApp {
    pub(crate) config: crate::data::Config,
    pub(crate) table: egui_data_table::DataTable<Row>,
    pub(crate) viewer: Viewer,
    pub(crate) data_sources: Vec<DataSource>,
    pub(crate) selected_index: Option<usize>,
    pub(crate) style_override: egui_data_table::Style,
    pub(crate) scroll_bar_always_visible: bool,
    pub(crate) pending_file_to_add: Option<std::path::PathBuf>,
    pub(crate) renaming_item: Option<(RenamingTarget, String)>,
    pub(crate) save_requested: Option<usize>,
}

impl Default for CorrelateApp {

    fn default() -> Self {
        let config_path = "config.json";
        let config = crate::data::Config::load(config_path).unwrap_or_default();

        let mut data_sources = Vec::new();
        for source in &config.data_sources {
            let extension = std::path::Path::new(source)
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("");

            let loaded = if extension == "csv" {
                match crate::data::load_csv(source) {
                    Ok(csv_sheet) => {
                        let sheets = vec![DataSheet {
                            name: csv_sheet.name,
                            display_name: csv_sheet.display_name,
                            column_configs: csv_sheet.column_configs,
                            table: csv_sheet.rows.into_iter().collect(),
                        }];
                        Ok((csv_sheet.custom_name, sheets))
                    }
                    Err(e) => Err(e),
                }
            } else {
                match crate::data::load_xlsx(source) {
                    Ok(excel_sheets) => {
                        let custom_name = excel_sheets.first().and_then(|s| s.custom_name.clone());
                        let sheets = excel_sheets.into_iter().map(|s| DataSheet {
                            name: s.name,
                            display_name: s.display_name,
                            column_configs: s.column_configs,
                            table: s.rows.into_iter().collect(),
                        }).collect();
                        Ok((custom_name, sheets))
                    }
                    Err(e) => Err(e),
                }
            };

            match loaded {
                Ok((custom_name, sheets)) => {
                    data_sources.push(DataSource {
                        path: source.to_string(),
                        name: custom_name,
                        sheets,
                        selected_sheet_index: 0,
                    });
                }
                Err(e) => {
                    log::error!("Failed to load {}: {}", source, e);
                }
            }
        }

        if data_sources.is_empty() {
            let selected_index = None;
            let table = egui_data_table::DataTable::new();
            let viewer = Viewer {
                name_filter: String::new(),
                row_protection: false,
                hotkeys: Vec::new(),
                captured_order: Vec::new(),
                add_column_requested: None,
                rename_row_requested: None,
                rename_column_requested: None,
                renaming_item: None,
                rename_committed: false,
                save_requested: false,
                column_configs: Vec::new(),
            };

            return Self {
                config,
                table,
                viewer,
                data_sources,
                selected_index,
                style_override: Default::default(),
                scroll_bar_always_visible: false,
                pending_file_to_add: None,
                renaming_item: None,
                save_requested: None,
            };
        }

        let selected_index = config.selected_index.unwrap_or(0).min(data_sources.len() - 1);
        let ds = &data_sources[selected_index];
        let sheet = &ds.sheets[ds.selected_sheet_index];
        let table = sheet.table.clone();
        let viewer = Viewer {
            name_filter: String::new(),
            row_protection: false,
            hotkeys: Vec::new(),
            captured_order: Vec::new(),
            add_column_requested: None,
            rename_row_requested: None,
            rename_column_requested: None,
            renaming_item: None,
            rename_committed: false,
            save_requested: false,
            column_configs: sheet.column_configs.clone(),
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
            renaming_item: None,
            save_requested: None,
        }
    }
}

impl eframe::App for CorrelateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Assert Send/Sync for DataTable as a compile-time check
        fn is_send<T: Send>(_: &T) {}
        fn is_sync<T: Sync>(_: &T) {}
        is_send(&self.table);
        is_sync(&self.table);

        self.ui_menu_bar(ctx);
        self.ui_bottom_panel(ctx);

        let (newly_selected_index, newly_selected_sheet_index): (Option<usize>, Option<usize>) = self.ui_hierarchy_panel(ctx);

        if let Some(index) = newly_selected_index {
            let sheet_idx = newly_selected_sheet_index.unwrap_or(0);
            self.switch_to_source(index, sheet_idx);
        }

        self.handle_pending_file_add();
        CentralPanel::default().ui(self, ctx);

        if let Some(index) = self.save_requested.take() {
            self.save_source_config(index);
        }
    }
}