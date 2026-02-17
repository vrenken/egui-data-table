use egui::{Response, Ui};
use crate::data::*;
use super::ColumnTypeEditor;

pub struct DateTimeEditor;
impl ColumnTypeEditor for DateTimeEditor {
    fn show(
        &self,
        ui: &mut Ui,
        cell_value: &mut CellValue,
        _column_config: &mut ColumnConfig,
    ) -> Option<Response> {
        Some(egui::TextEdit::singleline(&mut cell_value.0)
            .show(ui)
            .response)
    }
}
