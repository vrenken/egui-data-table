use std::path::Path;
use umya_spreadsheet::*;
use crate::data::{CellValue, ColumnConfig, ColumnType, Gender, Grade, Row};

pub struct ExcelSheet {
    pub name: String,
    pub column_configs: Vec<ColumnConfig>,
    pub rows: Vec<Row>,
}

pub fn load_xlsx<P: AsRef<Path>>(path: P) -> Result<Vec<ExcelSheet>, String> {
    let book = reader::xlsx::read(path).map_err(|e| e.to_string())?;
    let mut sheets = Vec::new();

    for sheet_idx in 0..book.get_sheet_count() {
        let sheet = book.get_sheet(&sheet_idx).ok_or(format!("Sheet {} not found", sheet_idx))?;
        let sheet_name = sheet.get_name().to_string();

        let (max_col, max_row) = sheet.get_highest_column_and_row();

        let mut column_configs = Vec::new();

        // 1. Infer ColumnConfig from the first row (headers)
        for col_idx in 1..=max_col {
            let col_name = sheet.get_formatted_value((col_idx, 1));
            
            // Infer type from the second row (first data row)
            let first_data_value = sheet.get_formatted_value((col_idx, 2));
            let column_type = infer_column_type(&col_name, &first_data_value);

            column_configs.push(ColumnConfig {
                name: col_name,
                column_type,
                is_sortable: true,
            });
        }

        // 2. Load data rows
        let mut rows = Vec::new();
        for row_idx in 2..=max_row {
            let mut cells = Vec::new();
            for col_idx in 1..=max_col {
                let value = sheet.get_formatted_value((col_idx, row_idx));
                let config = &column_configs[(col_idx - 1) as usize];
                cells.push(map_cell_value(&value, config.column_type));
            }
            rows.push(Row { cells });
        }

        sheets.push(ExcelSheet {
            name: sheet_name,
            column_configs,
            rows,
        });
    }

    Ok(sheets)
}

fn infer_column_type(name: &str, sample_value: &str) -> ColumnType {
    let name_lower = name.to_lowercase();
    if name_lower.contains("gender") {
        return ColumnType::Gender;
    }
    if name_lower.contains("grade") {
        return ColumnType::Grade;
    }
    if name_lower.contains("is student") || name_lower.contains("locked") {
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
        ColumnType::Gender => {
            let gender = match value.to_lowercase().as_str() {
                "male" | "m" => Some(Gender::Male),
                "female" | "f" => Some(Gender::Female),
                _ => None,
            };
            CellValue::Gender(gender)
        }
        ColumnType::Bool => {
            let b = match value.to_lowercase().as_str() {
                "true" | "1" | "yes" | "y" => true,
                _ => false,
            };
            CellValue::Bool(b)
        }
        ColumnType::Grade => {
            let grade = value.parse::<Grade>().unwrap_or(Grade::F);
            CellValue::Grade(grade)
        }
    }
}
