use crate::view::app::types::{DataSheet, DataSource};
use crate::view::CorrelateApp;

impl CorrelateApp {
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
}
