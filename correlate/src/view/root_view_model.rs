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
}

impl RootViewModel {
    pub fn save_source_config(&mut self, index: usize) {
        if let Some(ds) = self.data_sources.get_mut(index) {
            let companion_path = crate::data::SourceConfig::get_companion_path(&ds.path);
            
            // If the data source being saved is the currently selected one, 
            // update its internal state from the viewer and table first.
            if Some(index) == self.selected_index {
                ds.sheets[ds.selected_sheet_index].column_configs = self.viewer.column_configs.clone();
                ds.sheets[ds.selected_sheet_index].table = self.table.clone();
            }

            let mut sheet_configs = Vec::new();
            for sheet in &mut ds.sheets {
                for (i, config) in sheet.column_configs.iter_mut().enumerate() {
                    config.order = i;
                }

                let key_col_idx = sheet.column_configs.iter().position(|c| c.is_key);
                let virtual_cols: Vec<usize> = sheet.column_configs.iter().enumerate()
                    .filter(|(_, c)| c.is_virtual)
                    .map(|(i, _)| i)
                    .collect();

                let mut cell_values = Vec::new();
                if let Some(key_idx) = key_col_idx {
                    let rows: &Vec<Row> = &sheet.table;
                    for row in rows {
                        let key = row.cells[key_idx].0.clone();
                        if key.is_empty() {
                            continue;
                        }

                        for &v_idx in &virtual_cols {
                            let value = row.cells[v_idx].0.clone();
                            if !value.is_empty() {
                                cell_values.push(crate::data::CellValueConfig {
                                    key: key.clone(),
                                    column_name: sheet.column_configs[v_idx].name.clone(),
                                    value,
                                });
                            }
                        }
                    }
                }

                sheet_configs.push(crate::data::SheetConfig {
                    name: sheet.name.clone(),
                    display_name: sheet.display_name.clone(),
                    column_configs: sheet.column_configs.clone(),
                    sort_config: None,
                    cell_values,
                });
            }

            let source_config = crate::data::SourceConfig {
                name: ds.name.clone(),
                sheets: sheet_configs,
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

            let loaded_sheets = if extension == "csv" {
                match crate::data::load_csv(&path_str) {
                    Ok(csv_sheet) => Ok((csv_sheet.custom_name, vec![DataSheet {
                        name: csv_sheet.name,
                        display_name: csv_sheet.display_name,
                        column_configs: csv_sheet.column_configs,
                        table: csv_sheet.rows.into_iter().collect(),
                    }])),
                    Err(e) => Err(e),
                }
            } else {
                match crate::data::load_xlsx(&path_str) {
                    Ok(excel_sheets) => {
                        let custom_name = excel_sheets.first().and_then(|s| s.custom_name.clone());
                        Ok((custom_name, excel_sheets.into_iter().map(|s| DataSheet {
                            name: s.name,
                            display_name: s.display_name,
                            column_configs: s.column_configs,
                            table: s.rows.into_iter().collect(),
                        }).collect()))
                    },
                    Err(e) => Err(e),
                }
            };

            match loaded_sheets {
                Ok((custom_name, sheets)) => {
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
        self.viewer.column_configs = sheet.column_configs.clone();
    }

    pub fn save_datasource_configuration(&mut self) {
        if let Some(idx) = self.selected_index {
            let ds = &mut self.data_sources[idx];
            let sheet = &mut ds.sheets[ds.selected_sheet_index];
            sheet.column_configs = self.viewer.column_configs.clone();
            for (i, config) in sheet.column_configs.iter_mut().enumerate() {
                config.order = i;
            }
            sheet.table = self.table.clone();

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
                    let column_type = self.viewer.column_configs[name_col_idx].column_type;
                    row.cells[name_col_idx] = crate::data::map_cell_value(&new_name, column_type);
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