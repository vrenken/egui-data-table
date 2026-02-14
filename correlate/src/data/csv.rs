use std::path::Path;
use csv::ReaderBuilder;
use crate::data::{CellValue, ColumnConfig, ColumnType, Row, SheetConfig, SourceConfig};

pub struct CsvSheet {
    pub name: String,
    pub column_configs: Vec<ColumnConfig>,
    pub rows: Vec<Row>,
}

pub fn load_csv<P: AsRef<Path>>(path: P) -> Result<CsvSheet, String> {
    let file_name = path.as_ref().file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("CSV Data")
        .to_string();

    let companion_path = SourceConfig::get_companion_path(&path);
    let source_config = SourceConfig::load(&companion_path).ok();

    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path(&path)
        .map_err(|e| e.to_string())?;

    let headers = reader.headers().map_err(|e| e.to_string())?.clone();
    
    let mut column_configs = source_config.as_ref()
        .and_then(|sc| sc.sheets.iter().find(|s| s.name == file_name))
        .map(|s| s.column_configs.clone())
        .unwrap_or_default();

    // If not loaded from config, infer them
    if column_configs.is_empty() {
        // We need a sample to infer types. Let's read the first record.
        let mut temp_reader = ReaderBuilder::new()
            .has_headers(true)
            .from_path(&path)
            .map_err(|e| e.to_string())?;
        
        let first_record = temp_reader.records().next();
        
        for (i, header) in headers.iter().enumerate() {
            let sample_value = first_record.as_ref()
                .and_then(|r| r.as_ref().ok())
                .and_then(|r| r.get(i))
                .unwrap_or("");
            
            let column_type = infer_column_type(header, sample_value);
            column_configs.push(ColumnConfig {
                name: header.to_string(),
                column_type,
                is_sortable: true,
                is_key: false,
                is_virtual: false,
                width: None,
            });
        }
    }

    let mut rows = Vec::new();
    for result in reader.records() {
        let record = result.map_err(|e| e.to_string())?;
        let mut cells = Vec::new();
        let mut physical_col_idx = 0;
        for config in &column_configs {
            if config.is_virtual {
                cells.push(CellValue::String("".to_string()));
            } else {
                if let Some(value) = record.get(physical_col_idx) {
                    cells.push(map_cell_value(value, config.column_type));
                }
                physical_col_idx += 1;
            }
        }
        rows.push(Row { cells });
    }

    // Save companion file if it didn't exist
    if source_config.is_none() {
        let new_config = SourceConfig {
            sheets: vec![SheetConfig {
                name: file_name.clone(),
                column_configs: column_configs.clone(),
                sort_config: None, // Will be updated by UI
            }]
        };
        if let Err(e) = new_config.save(&companion_path) {
            log::error!("Failed to save companion config to {:?}: {}", companion_path, e);
        }
    }

    Ok(CsvSheet {
        name: file_name,
        column_configs,
        rows,
    })
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
