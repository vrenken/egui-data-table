use egui_data_table::viewer::{DecodeErrorBehavior, RowCodec};
use crate::data::{AGE, GRADE, IS_STUDENT, NAME, ROW_LOCKED};
use crate::data::row::{Row, CellValue};

pub struct Codec;

impl RowCodec<Row> for Codec {
    type DeserializeError = &'static str;

    fn create_empty_decoded_row(&mut self) -> Row {
        Row::default()
    }

    fn encode_column(&mut self, src_row: &Row, column: usize, dst: &mut String) {
        match &src_row.cells[column] {
            CellValue::String(s) => dst.push_str(s),
            CellValue::Int(i) => dst.push_str(&i.to_string()),
            CellValue::Bool(b) => dst.push_str(&b.to_string()),
            CellValue::Grade(g) => dst.push_str(&g.to_string()),
            CellValue::Gender(g) => {
                if let Some(gender) = g {
                    dst.push_str(&gender.to_string());
                }
            }
        }
    }

    fn decode_column(
        &mut self,
        src_data: &str,
        column: usize,
        dst_row: &mut Row,
    ) -> Result<(), DecodeErrorBehavior> {
        match column {
            NAME => {
                if let CellValue::String(ref mut s) = dst_row.cells[NAME] {
                    s.replace_range(.., src_data);
                }
            }
            AGE => {
                if let CellValue::Int(ref mut i) = dst_row.cells[AGE] {
                    *i = src_data.parse().map_err(|_| DecodeErrorBehavior::SkipRow)?;
                }
            }
            IS_STUDENT => {
                if let CellValue::Bool(ref mut b) = dst_row.cells[IS_STUDENT] {
                    *b = src_data.parse().map_err(|_| DecodeErrorBehavior::SkipRow)?;
                }
            }
            GRADE => {
                if let CellValue::Grade(ref mut g) = dst_row.cells[GRADE] {
                    *g = src_data.parse().map_err(|_| DecodeErrorBehavior::SkipRow)?;
                }
            }
            ROW_LOCKED => {
                if let CellValue::Bool(ref mut b) = dst_row.cells[ROW_LOCKED] {
                    *b = src_data.parse().map_err(|_| DecodeErrorBehavior::SkipRow)?;
                }
            }
            _ => return Err(DecodeErrorBehavior::SkipRow),
        }

        Ok(())
    }
}