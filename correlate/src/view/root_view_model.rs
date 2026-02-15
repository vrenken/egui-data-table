use crate::data::{Config, Row};
use crate::data::{DataSheet, DataSource, RenamingTarget};
use crate::view::RowView;

pub struct RootViewModel {
    pub(crate) config: crate::data::Config,
    pub(crate) table: egui_data_table::DataTable<Row>,
    pub(crate) viewer: RowView,
    pub(crate) data_sources: Vec<DataSource>,
    pub(crate) selected_index: Option<usize>,
    pub(crate) style_override: egui_data_table::Style,
    pub(crate) scroll_bar_always_visible: bool,
    pub(crate) pending_file_to_add: Option<std::path::PathBuf>,
    pub(crate) renaming_item: Option<(RenamingTarget, String)>,
    pub(crate) save_requested: Option<usize>,
}

impl RootViewModel {
    pub fn save_source_config(&mut self, index: usize) {
        if let Some(ds) = self.data_sources.get_mut(index) {
            let companion_path = crate::data::SourceConfig::get_companion_path(&ds.path);
            for sheet in &mut ds.sheets {
                for (i, config) in sheet.column_configs.iter_mut().enumerate() {
                    config.order = i;
                }
            }
            let source_config = crate::data::SourceConfig {
                name: ds.name.clone(),
                sheets: ds.sheets.iter().map(|s| crate::data::SheetConfig {
                    name: s.name.clone(),
                    display_name: s.display_name.clone(),
                    column_configs: s.column_configs.clone(),
                    sort_config: None,
                }).collect(),
            };
            if let Err(e) = source_config.save(companion_path) {
                log::error!("Failed to save companion config for {}: {}", ds.path, e);
            }
        }
    }

    pub fn default(config: Config) -> Self {

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
            let viewer = RowView {
                name_filter: String::new(),
                row_protection: false,
                hotkeys: Vec::new(),
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
        let viewer = RowView {
            name_filter: String::new(),
            row_protection: false,
            hotkeys: Vec::new(),
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