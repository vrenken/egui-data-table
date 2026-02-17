use egui::{Response, Ui};
use crate::data::*;
use super::ColumnTypeEditor;

pub struct MultiSelectEditor;
impl ColumnTypeEditor for MultiSelectEditor {
    fn show(
        &self,
        ui: &mut Ui,
        cell_value: &mut CellValue,
        _column_config: &mut ColumnConfig,
        _data_sources: &[DataSource],
    ) -> Option<Response> {
        Some(ui.text_edit_singleline(&mut cell_value.0))
    }
}
