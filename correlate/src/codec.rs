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
                ColumnType::String => CellValue::String("".to_string()),
                ColumnType::Int => CellValue::Int(0),
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
                CellValue::Int(i) => dst.push_str(&i.to_string()),
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
            ColumnType::String => {
                if let CellValue::String(ref mut s) = dst_row.cells[column] {
                    s.replace_range(.., src_data);
                }
            }
            ColumnType::Int => {
                if let CellValue::Int(ref mut i) = dst_row.cells[column] {
                    *i = src_data.parse().map_err(|_| DecodeErrorBehavior::SkipRow)?;
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