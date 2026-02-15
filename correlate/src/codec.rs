use egui_data_table::viewer::{DecodeErrorBehavior, RowCodec};
use crate::data::row::{CellValue, Row};
use crate::data::column_config::ColumnConfig;
use crate::data::column_type::ColumnType;

pub struct Codec {
    pub column_configs: Vec<ColumnConfig>,
}

impl RowCodec<Row> for Codec {
    type DeserializeError = &'static str;

    fn create_empty_decoded_row(&mut self) -> Row {
        let mut cells = Vec::with_capacity(self.column_configs.len());
        for config in &self.column_configs {
            let cell = match config.column_type {
                ColumnType::Text => CellValue::String("".to_string()),
                ColumnType::Number => CellValue::Number(0.0),
                ColumnType::DateTime => CellValue::DateTime("".to_string()),
                ColumnType::Bool => CellValue::Bool(false),
            };
            cells.push(cell);
        }
        Row { cells }
    }

    fn encode_column(&mut self, src_row: &Row, column: usize, dst: &mut String) {
        if let Some(cell) = src_row.cells.get(column) {
            match cell {
                CellValue::String(s) => dst.push_str(s),
                CellValue::Number(n) => dst.push_str(&n.to_string()),
                CellValue::DateTime(dt) => dst.push_str(dt),
                CellValue::Bool(b) => dst.push_str(&b.to_string()),
            }
        }
    }

    fn decode_column(
        &mut self,
        src_data: &str,
        column: usize,
        dst_row: &mut Row,
    ) -> Result<(), DecodeErrorBehavior> {
        let config = self.column_configs.get(column).ok_or(DecodeErrorBehavior::SkipRow)?;
        
        match config.column_type {
            ColumnType::Text => {
                if let CellValue::String(ref mut s) = dst_row.cells[column] {
                    s.replace_range(.., src_data);
                }
            }
            ColumnType::Number => {
                if let CellValue::Number(ref mut n) = dst_row.cells[column] {
                    *n = src_data.parse().map_err(|_| DecodeErrorBehavior::SkipRow)?;
                }
            }
            ColumnType::DateTime => {
                if let CellValue::DateTime(ref mut dt) = dst_row.cells[column] {
                    dt.replace_range(.., src_data);
                }
            }
            ColumnType::Bool => {
                if let CellValue::Bool(ref mut b) = dst_row.cells[column] {
                    *b = src_data.parse().map_err(|_| DecodeErrorBehavior::SkipRow)?;
                }
            }
        }

        Ok(())
    }
}