use egui_data_table::viewer::{DecodeErrorBehavior, RowCodec};
use crate::data::*;

pub struct Codec {
    pub column_configs: Vec<ColumnConfiguration>,
}

impl RowCodec<Row> for Codec {
    type DeserializeError = &'static str;

    fn create_empty_decoded_row(&mut self) -> Row {
        let cells = self.column_configs.iter()
            .map(|config| config.column_type.default_value())
            .collect();
        Row { cells }
    }

    fn encode_column(&mut self, src_row: &Row, column: usize, dst: &mut String) {
        if let Some(cell) = src_row.cells.get(column) {
            dst.push_str(&cell.0);
        }
    }

    fn decode_column(
        &mut self,
        src_data: &str,
        column: usize,
        dst_row: &mut Row,
    ) -> Result<(), DecodeErrorBehavior> {
        if let Some(cell) = dst_row.cells.get_mut(column) {
            cell.0.replace_range(.., src_data);
            Ok(())
        } else {
            Err(DecodeErrorBehavior::SkipRow)
        }
    }
}