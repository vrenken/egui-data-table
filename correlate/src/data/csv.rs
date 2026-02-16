use std::path::Path;
use csv::ReaderBuilder;
use crate::data::{CellValue, ColumnConfig, Row, SheetConfig, SourceConfig, infer_column_type, map_cell_value};

pub struct CsvSheet {
    pub name: String,
    pub custom_name: Option<String>,
    pub display_name: Option<String>,
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
    
    let custom_name = source_config.as_ref().and_then(|sc| sc.name.clone());

    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path(&path)
        .map_err(|e| e.to_string())?;

    let headers = reader.headers().map_err(|e| e.to_string())?.clone();
    
    let config_sheet = source_config.as_ref()
        .and_then(|sc| sc.sheets.iter().find(|s| s.name == file_name));

    let mut column_configs = config_sheet
        .map(|s| s.column_configs.clone())
        .unwrap_or_default();

    let sheet_display_name = config_sheet.and_then(|s| s.display_name.clone());

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
                display_name: None,
                column_type,
                is_key: false,
                is_name: false,
                is_virtual: false,
                order: i,
                width: None,
                allowed_values: None,
            });
        }
    } else {
        column_configs.sort_by_key(|c| c.order);
    }

    let mut rows = Vec::new();
    let cell_values = config_sheet.map(|s| &s.cell_values);

    for result in reader.records() {
        let record = result.map_err(|e| e.to_string())?;
        
        // 1. First pass: get the physical key value if it exists
        let mut row_key = None;
        let mut phys_idx = 0;
        for config in &column_configs {
            if !config.is_virtual {
                if config.is_key {
                    row_key = record.get(phys_idx).map(|v| v.to_string());
                }
                phys_idx += 1;
            }
        }

        // 2. Second pass: build the row
        let mut cells = Vec::new();
        let mut phys_idx = 0;
        for config in &column_configs {
            if config.is_virtual {
                let mut val = "".to_string();
                if let (Some(key), Some(stored_values)) = (&row_key, cell_values) {
                    if let Some(stored) = stored_values.iter().find(|cv| cv.key == *key && cv.column_name == config.name) {
                        val = stored.value.clone();
                    }
                }
                cells.push(CellValue(val));
            } else {
                if let Some(value) = record.get(phys_idx) {
                    cells.push(map_cell_value(value, config.column_type));
                }
                phys_idx += 1;
            }
        }
        rows.push(Row { cells });
    }

    // Save companion file if it didn't exist
    if source_config.is_none() {
        let new_config = SourceConfig {
            name: None,
            sheets: vec![SheetConfig {
                name: file_name.clone(),
                display_name: sheet_display_name.clone(),
                column_configs: column_configs.clone(),
                sort_config: None, // Will be updated by UI
                cell_values: Vec::new(),
            }]
        };
        if let Err(e) = new_config.save(&companion_path) {
            log::error!("Failed to save companion config to {:?}: {}", companion_path, e);
        }
    }

    Ok(CsvSheet {
        name: file_name,
        custom_name,
        display_name: sheet_display_name,
        column_configs,
        rows,
    })
}
