use crate::data::{Config, RenamingTarget, Row};
use crate::data::*;
use crate::view::*;

pub struct RootViewModel {
    pub(crate) config: Config,
    pub(crate) table: egui_data_table::DataTable<Row>,
    pub(crate) viewer: RowView,
    pub(crate) data_sources: Vec<DataSource>,
    pub(crate) selected_index: Option<usize>,
    pub(crate) style_override: egui_data_table::Style,
    pub(crate) scroll_bar_always_visible: bool,
    pub(crate) pending_file_to_add: Option<std::path::PathBuf>,
}

impl RootViewModel {
    pub fn save_source_config(&mut self, index: usize) {
        if let Some(ds) = self.data_sources.get_mut(index) {
            let (configs, table) = if Some(index) == self.selected_index {
                (self.viewer.column_configs.clone(), self.table.clone())
            } else {
                let sheet = &ds.sheets[ds.selected_sheet_index];
                (sheet.column_configs.clone(), sheet.table.clone())
            };

            if let Err(e) = ds.save(configs, table) {
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


            let loader: Box<dyn SheetLoader> = if extension == "csv" {
                Box::new(CsvSheet)
            } else {
                Box::new(ExcelSheet)
            };

            let loaded = loader.load(source.clone());

            match loaded {
                Ok(loaded_sheets) => {
                    let custom_name = loaded_sheets.first().and_then(|s| s.custom_name.clone());
                    data_sources.push(DataSource {
                        path: source.to_string(),
                        name: custom_name,
                        sheets: loaded_sheets,
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
                column_configs: Vec::new(),
                config: config.clone(),
                data_sources: data_sources.clone(),
                visible_columns: None,
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
            column_configs: sheet.column_configs.clone(),
            config: config.clone(),
            data_sources: data_sources.clone(),
            visible_columns: None,
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

    pub fn handle_pending_file_add(&mut self) {
        if let Some(path) = self.pending_file_to_add.take() {
            let path_str = path.to_string_lossy().to_string();
            let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");

            let loaded_result = if extension == "csv" {
                crate::data::CsvSheet.load(path_str.clone())
            } else {
                crate::data::ExcelSheet.load(path_str.clone())
            };

            match loaded_result {
                Ok(sheets) => {
                    let custom_name = sheets.first().and_then(|s| s.custom_name.clone());
                    let new_index = self.data_sources.len();

                    self.data_sources.push(DataSource {
                        path: path_str.clone(),
                        name: custom_name,
                        sheets,
                        selected_sheet_index: 0,
                    });

                    self.switch_to_source(new_index, 0);

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
    }

    pub fn switch_to_source(&mut self, index: usize, sheet_idx: usize) {
        // Save current table state back to its source
        if let Some(old_idx) = self.selected_index {
            let old_ds = &mut self.data_sources[old_idx];
            let old_sheet = &mut old_ds.sheets[old_ds.selected_sheet_index];
            old_sheet.table = self.table.clone();
            old_sheet.column_configs = self.viewer.column_configs.clone();

            self.save_source_config(old_idx);
        }

        // Switch to new source
        self.selected_index = Some(index);
        let ds = &mut self.data_sources[index];
        ds.selected_sheet_index = sheet_idx;
        let sheet = &ds.sheets[sheet_idx];
        self.table = sheet.table.clone();
        self.viewer.config = self.config.clone();
        self.viewer.column_configs = sheet.column_configs.clone();
        self.viewer.data_sources = self.data_sources.clone();
    }

    pub fn save_datasource_configuration(&mut self) {
        if let Some(idx) = self.selected_index {
            self.save_source_config(idx);
        }
    }

    pub fn apply_rename(&mut self, target: RenamingTarget, new_name: String) {
        match target {
            RenamingTarget::DataSource(ds_idx) => {
                if let Some(ds) = self.data_sources.get_mut(ds_idx) {
                    ds.name = if new_name.is_empty() { None } else { Some(new_name) };
                }
            }
            RenamingTarget::Sheet(ds_idx, sheet_idx) => {
                if let Some(ds) = self.data_sources.get_mut(ds_idx) {
                    if let Some(sheet) = ds.sheets.get_mut(sheet_idx) {
                        sheet.display_name = if new_name.is_empty() || new_name == sheet.name { None } else { Some(new_name) };
                    }
                }
            }
            RenamingTarget::Row(row_idx) => {
                let name_col_idx = self.viewer.column_configs.iter().position(|c| c.is_name)
                    .or_else(|| self.viewer.column_configs.iter().position(|c| c.name.contains("Name")))
                    .or_else(|| self.viewer.column_configs.iter().position(|c| c.column_type == crate::data::ColumnType::Text))
                    .unwrap_or(0);

                if let Some(row) = self.table.get_mut(row_idx) {
                    row.cells[name_col_idx] = CellValue::from(new_name);
                }
            }
            RenamingTarget::Column(col_idx) => {
                if let Some(config) = self.viewer.column_configs.get_mut(col_idx) {
                    config.display_name = if new_name.is_empty() || new_name == config.name { None } else { Some(new_name) };
                    if config.is_virtual {
                        config.name = config.display_name.clone().unwrap_or_else(|| config.name.clone());
                    }
                }
            }
        }
        self.save_datasource_configuration();
    }
}