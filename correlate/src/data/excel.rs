use std::path::Path;
use umya_spreadsheet::*;
use crate::data::{CellValue, ColumnConfig, Row, SheetConfig, SourceConfig, infer_column_type, map_cell_value};

pub struct ExcelSheet {
    pub name: String,
    pub custom_name: Option<String>,
    pub display_name: Option<String>,
    pub column_configs: Vec<ColumnConfig>,
    pub rows: Vec<Row>,
}

pub fn load_xlsx<P: AsRef<Path>>(path: P) -> Result<Vec<ExcelSheet>, String> {
    let book = reader::xlsx::read(&path).map_err(|e| e.to_string())?;
    
    let companion_path = SourceConfig::get_companion_path(&path);
    let source_config = SourceConfig::load(&companion_path).ok();
    
    let custom_name = source_config.as_ref().and_then(|sc| sc.name.clone());
    let mut sheets = Vec::new();
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
                let column_type = infer_column_type(&col_name, &first_data_value);

                column_configs.push(ColumnConfig {
                    name: col_name,
                    display_name: None,
                    column_type,
                    is_key: false,
                    is_name: false,
                    is_virtual: false,
                    order: col_idx as usize - 1,
                    width: None,
                    allowed_values: None,
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
                if config.is_virtual {
                    let mut val = "".to_string();
                    if let (Some(key), Some(stored_values)) = (&row_key, cell_values) {
                        if let Some(stored) = stored_values.iter().find(|cv| cv.key == *key) {
                            val = stored.value.clone();
                        }
                    }
                    cells.push(CellValue(val));
                } else {
                    let value = sheet.get_formatted_value((col_idx as u32 + 1, row_idx));
                    cells.push(map_cell_value(&value, config.column_type));
                }
            }
            rows.push(Row { cells });
        }

        sheets.push(ExcelSheet {
            name: sheet_name,
            custom_name: custom_name.clone(),
            display_name: sheet_display_name,
            column_configs,
            rows,
        });
    }

    // Save companion file if it didn't exist
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
