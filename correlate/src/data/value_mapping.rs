use crate::data::{CellValue, ColumnType};

pub fn infer_column_type(name: &str, sample_value: &str) -> ColumnType {
    let name_lower = name.to_lowercase();
    if name_lower.contains("locked") {
        return ColumnType::Bool;
    }

    // Try to parse sample value
    if sample_value.parse::<i32>().is_ok() {
        return ColumnType::Int;
    }
    if sample_value.parse::<f64>().is_ok() {
        return ColumnType::Float;
    }
    // Check for DateTime (simple heuristic for common formats)
    if is_datetime(sample_value) {
        return ColumnType::DateTime;
    }
    if sample_value.parse::<bool>().is_ok() {
        return ColumnType::Bool;
    }
    
    ColumnType::String
}

pub fn map_cell_value(value: &str, column_type: ColumnType) -> CellValue {
    match column_type {
        ColumnType::String => CellValue::String(value.to_string()),
        ColumnType::Int => CellValue::Int(value.parse().unwrap_or(0)),
        ColumnType::Float => CellValue::Float(value.parse().unwrap_or(0.0)),
        ColumnType::DateTime => CellValue::DateTime(value.to_string()),
        ColumnType::Bool => {
            let b = match value.to_lowercase().as_str() {
                "true" | "1" | "yes" | "y" => true,
                _ => false,
            };
            CellValue::Bool(b)
        }
    }
}

fn is_datetime(s: &str) -> bool {
    if s.is_empty() { return false; }
    
    // Check for common ISO-like formats: 2024-02-14, 2024/02/14, 14-02-2024, etc.
    // This is a very basic check.
    let has_date_separators = s.contains('-') || s.contains('/') || s.contains('.');
    let _has_time_separators = s.contains(':');
    
    if has_date_separators {
        // Look for digit-heavy string
        let digit_count = s.chars().filter(|c| c.is_ascii_digit()).count();
        if digit_count >= 6 {
            // Further refinement: check if it starts with year or ends with year
            // or has time component
            return true;
        }
    }
    
    false
}
