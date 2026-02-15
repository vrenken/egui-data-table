use crate::data::{DataSheet, DataSource};
use crate::view::RootView;

impl RootView {

    pub fn switch_to_source(&mut self, index: usize, sheet_idx: usize) {
        // Save current table state back to its source
        if let Some(old_idx) = self.view_model.selected_index {
            let old_ds = &mut self.view_model.data_sources[old_idx];
            let old_sheet = &mut old_ds.sheets[old_ds.selected_sheet_index];
            old_sheet.table = self.view_model.table.clone();
            old_sheet.column_configs = self.view_model.viewer.column_configs.clone();
            
            self.view_model.save_source_config(old_idx);
        }

        // Switch to new source
        self.view_model.selected_index = Some(index);
        let ds = &mut self.view_model.data_sources[index];
        ds.selected_sheet_index = sheet_idx;
        let sheet = &ds.sheets[sheet_idx];
        self.view_model.table = sheet.table.clone();
        self.view_model.viewer.column_configs = sheet.column_configs.clone();
    }

    pub fn handle_pending_file_add(&mut self) {
        if let Some(path) = self.view_model.pending_file_to_add.take() {
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
                    let new_index = self.view_model.data_sources.len();
                    
                    self.view_model.data_sources.push(DataSource {
                        path: path_str.clone(),
                        name: custom_name,
                        sheets,
                        selected_sheet_index: 0,
                    });
                    
                    self.switch_to_source(new_index, 0);

                    // Persist to config
                    self.view_model.config.data_sources = self.view_model.data_sources.iter().map(|ds| ds.path.clone()).collect();
                    self.view_model.config.selected_index = self.view_model.selected_index;
                    let config_path = "config.json";
                    if let Err(e) = self.view_model.config.save(config_path) {
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
