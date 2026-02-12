use egui_data_table::viewer::{DecodeErrorBehavior, RowCodec};
use crate::data::{AGE, GRADE, IS_STUDENT, NAME, ROW_LOCKED};
use crate::data::row::Row;

pub struct Codec;

impl RowCodec<Row> for Codec {
    type DeserializeError = &'static str;

    fn create_empty_decoded_row(&mut self) -> Row {
        Row::default()
    }

    fn encode_column(&mut self, src_row: &Row, column: usize, dst: &mut String) {
        match column {
            NAME => dst.push_str(&src_row.name),
            AGE => dst.push_str(&src_row.age.to_string()),
            IS_STUDENT => dst.push_str(&src_row.is_student.to_string()),
            GRADE => dst.push_str(src_row.grade.to_string().as_str()),
            ROW_LOCKED => dst.push_str(&src_row.row_locked.to_string()),
            _ => unreachable!(),
        }
    }

    fn decode_column(
        &mut self,
        src_data: &str,
        column: usize,
        dst_row: &mut Row,
    ) -> Result<(), DecodeErrorBehavior> {
        match column {
            NAME => dst_row.name.replace_range(.., src_data),
            AGE => dst_row.age = src_data.parse().map_err(|_| DecodeErrorBehavior::SkipRow)?,
            IS_STUDENT => dst_row.is_student = src_data.parse().map_err(|_| DecodeErrorBehavior::SkipRow)?,
            GRADE => {
                dst_row.grade = src_data.parse().map_err(|_| DecodeErrorBehavior::SkipRow)?;
            }
            ROW_LOCKED => dst_row.row_locked = src_data.parse().map_err(|_| DecodeErrorBehavior::SkipRow)?,
            _ => unreachable!(),
        }

        Ok(())
    }
}