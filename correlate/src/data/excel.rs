use std::path::Path;
use umya_spreadsheet::*;
use crate::data::{CellValue, ColumnConfig, ColumnType, Row, SheetConfig, SourceConfig};

pub struct ExcelSheet {
    pub name: String,
    pub custom_name: Option<String>,
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

        let mut column_configs = source_config.as_ref()
            .and_then(|sc| sc.sheets.iter().find(|s| s.name == sheet_name))
            .map(|s| s.column_configs.clone())
            .unwrap_or_default();

        // If not loaded from config, infer them
        if column_configs.is_empty() {
            for col_idx in 1..=max_col {
                let col_name = sheet.get_formatted_value((col_idx, 1));
                
                // Infer type from the second row (first data row)
                let first_data_value = sheet.get_formatted_value((col_idx, 2));
                let column_type = infer_column_type(&col_name, &first_data_value);

                column_configs.push(ColumnConfig {
                    name: col_name,
                    column_type,
                    is_sortable: true,
                    is_key: false,
                    is_virtual: false,
                    width: None,
                });
            }
        }

        config_sheets.push(SheetConfig {
            name: sheet_name.clone(),
            column_configs: column_configs.clone(),
            sort_config: None,
        });

        // 2. Load data rows
        let mut rows = Vec::new();
        let max_col_idx = column_configs.len();
        for row_idx in 2..=max_row {
            let mut cells = Vec::new();
            for col_idx in 1..=max_col_idx {
                let config = &column_configs.get(col_idx - 1).unwrap();
                if config.is_virtual {
                    cells.push(CellValue::String("".to_string()));
                } else {
                    let value = sheet.get_formatted_value((col_idx as u32, row_idx));
                    cells.push(map_cell_value(&value, config.column_type));
                }
            }
            rows.push(Row { cells });
        }

        sheets.push(ExcelSheet {
            name: sheet_name,
            custom_name: custom_name.clone(),
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

fn infer_column_type(name: &str, sample_value: &str) -> ColumnType {
    let name_lower = name.to_lowercase();
    if name_lower.contains("locked") {
        return ColumnType::Bool;
    }

    // Try to parse sample value
    if sample_value.parse::<i32>().is_ok() {
        return ColumnType::Int;
    }
    if sample_value.parse::<bool>().is_ok() {
        return ColumnType::Bool;
    }
    
    ColumnType::String
}

fn map_cell_value(value: &str, column_type: ColumnType) -> CellValue {
    match column_type {
        ColumnType::String => CellValue::String(value.to_string()),
        ColumnType::Int => CellValue::Int(value.parse().unwrap_or(0)),
        ColumnType::Bool => {
            let b = match value.to_lowercase().as_str() {
                "true" | "1" | "yes" | "y" => true,
                _ => false,
            };
            CellValue::Bool(b)
        }
    }
}
