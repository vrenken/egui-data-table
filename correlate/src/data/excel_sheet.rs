use umya_spreadsheet::*;
use crate::data::{Row};
use crate::data::*;

pub struct ExcelSheet;

impl Default for ExcelSheet {
    fn default() -> Self {
        Self {
        }
    }   
}

impl SheetLoader for ExcelSheet {
    fn load(&self, path: String) -> Result<Vec<DataSheet>, String> {
        let book = reader::xlsx::read(&path).map_err(|e| e.to_string())?;
        
        let companion_path = DataSource::get_companion_path(&path);
        let source_config = SourceConfig::load(&companion_path).ok();
        
        let custom_name = source_config.as_ref().and_then(|sc| sc.name.clone());
        let mut sheets: Vec<DataSheet> = Vec::new();
        let mut config_sheets = Vec::new();

        for sheet_idx in 0..book.get_sheet_count() {
            let sheet = book.get_sheet(&sheet_idx).ok_or(format!("Sheet {} not found", sheet_idx))?;
            let sheet_name = sheet.get_name().to_string();

            let (max_col, max_row) = sheet.get_highest_column_and_row();

            let config_sheet = source_config.as_ref()
                .and_then(|sc| sc.sheets.iter().find(|s| s.name == sheet_name));

            let mut column_configs = config_sheet
                .map(|s| s.column_configs.clone())
                .unwrap_or_default();

            let sheet_display_name = config_sheet.and_then(|s| s.display_name.clone());

            // If not loaded from config, infer them
            if column_configs.is_empty() {
                for col_idx in 1..=max_col {
                    let col_name = sheet.get_formatted_value((col_idx, 1));
                    
                    // Infer type from the second row (first data row)
                    let first_data_value = sheet.get_formatted_value((col_idx, 2));
                    let column_type = ColumnType::infer(&col_name, &first_data_value);

                    column_configs.push(ColumnConfig {
                        name: col_name,
                        display_name: None,
                        column_type,
                        is_key: false,
                        is_name: false,
                        is_virtual: false,
                        is_visible: true,
                        order: col_idx as usize - 1,
                        width: None,
                        allowed_values: None,
                        related_source: None,
                    });
                }
            } else {
                column_configs.sort_by_key(|c| c.order);
            }

            config_sheets.push(SheetConfig {
                name: sheet_name.clone(),
                display_name: sheet_display_name.clone(),
                column_configs: column_configs.clone(),
                sort_config: None,
                cell_values: Vec::new(),
            });

            let mut rows = Vec::new();
            let cell_values = config_sheet.map(|s| &s.cell_values);

            for row_idx in 2..=max_row {
                // 1. First pass: get the physical key value if it exists
                let mut row_key = None;
                for (col_idx, config) in column_configs.iter().enumerate() {
                    if !config.is_virtual && config.is_key {
                        row_key = Some(sheet.get_formatted_value((col_idx as u32 + 1, row_idx)));
                        break;
                    }
                }

                // 2. Second pass: build the row
                let mut cells = Vec::new();
                for (col_idx, config) in column_configs.iter().enumerate() {
                    let physical_value = if !config.is_virtual {
                        Some(sheet.get_formatted_value((col_idx as u32 + 1, row_idx)))
                    } else {
                        None
                    };
                    
                    cells.push(config.column_type.load(
                        physical_value.as_deref(),
                        config,
                        row_key.as_deref(),
                        cell_values.map(|v| v.as_slice())
                    ));
                }
                rows.push(Row { cells });
            }

            sheets.push(DataSheet {
                name: sheet_name,
                custom_name: custom_name.clone(),
                display_name: sheet_display_name,
                icon: egui_material_icons::icons::ICON_TABLE_CHART,
                column_configs,
                table: rows.into_iter().collect(),
            });
        }

        // Save the companion file if it didn't exist
        if source_config.is_none() {
            let new_config = SourceConfig {
                name: None,
                sheets: config_sheets,
            };
            if let Err(e) = new_config.save(&companion_path) {
                log::error!("Failed to save companion config to {:?}: {}", companion_path, e);
            }
        }

        Ok(sheets)
    }
}
