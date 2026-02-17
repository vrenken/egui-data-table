use egui::{Response, Ui};
use crate::data::*;
use super::ColumnTypeEditor;

pub struct BoolEditor;
impl ColumnTypeEditor for BoolEditor {
    fn show(
        &self,
        ui: &mut Ui,
        cell_value: &mut CellValue,
        _column_config: &mut ColumnConfig,
    ) -> Option<Response> {
        let mut b: bool = cell_value.0.parse().unwrap_or(false);
        let res = ui.checkbox(&mut b, "");
        if res.changed() {
            cell_value.0 = b.to_string();
        }
        Some(res)
    }
}
