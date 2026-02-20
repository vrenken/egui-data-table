use egui::{Response, Ui};
use crate::data::*;
use crate::view::*;
use super::ColumnTypeEditor;

pub struct TextEditor;
impl ColumnTypeEditor for TextEditor {
    fn show(
        &self,
        ui: &mut Ui,
        cell_value: &mut CellValue,
        _column_config: &mut ColumnConfig,
        _view_model: &mut RootViewModel
    ) -> Option<Response> {
        Some(egui::TextEdit::multiline(&mut cell_value.0)
            .desired_rows(1)
            .code_editor()
            .show(ui)
            .response)
    }
}
