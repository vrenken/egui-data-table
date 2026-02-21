use egui::{Response, Ui};
use crate::data::*;
use crate::view::*;

pub struct NumberEditor;
impl ColumnTypeEditor for NumberEditor {
    fn show(
        &self,
        ui: &mut Ui,
        cell_value: &mut CellValue,
        _column_config: &mut ColumnConfiguration,
        _view_model: &mut RootViewModel
    ) -> Option<Response> {
        let mut n: f64 = cell_value.0.parse().unwrap_or(0.0);
        let res = ui.add(egui::DragValue::new(&mut n).speed(0.1));
        if res.changed() {
            cell_value.0 = n.to_string();
        }
        Some(res)
    }
}
