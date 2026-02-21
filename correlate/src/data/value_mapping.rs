use crate::data::*;

pub fn map_cell_value(value: &str, _column_type: crate::data::ColumnType) -> CellValue {
    CellValue(value.to_string())
}
