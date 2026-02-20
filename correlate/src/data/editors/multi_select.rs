use egui::{Response, Ui};
use crate::data::*;
use crate::view::*;
use super::ColumnTypeEditor;

pub struct MultiSelectEditor;
impl ColumnTypeEditor for MultiSelectEditor {
    fn show(
        &self,
        ui: &mut Ui,
        cell_value: &mut CellValue,
        _column_config: &mut ColumnConfig,
        _view_model: &mut RootViewModel
    ) -> Option<Response> {
        Some(ui.text_edit_singleline(&mut cell_value.0))
    }
}
