use egui::{Response, Ui};
use crate::data::{CellValue, ColumnConfig};

pub trait ColumnTypeEditor {
    fn show(
        &self,
        ui: &mut Ui,
        cell_value: &mut CellValue,
        column_config: &mut ColumnConfig,
    ) -> Option<Response>;
}

pub struct TextEditor;
impl ColumnTypeEditor for TextEditor {
    fn show(
        &self,
        ui: &mut Ui,
        cell_value: &mut CellValue,
        _column_config: &mut ColumnConfig,
    ) -> Option<Response> {
        Some(egui::TextEdit::multiline(&mut cell_value.0)
            .desired_rows(1)
            .code_editor()
            .show(ui)
            .response)
    }
}

pub struct NumberEditor;
impl ColumnTypeEditor for NumberEditor {
    fn show(
        &self,
        ui: &mut Ui,
        cell_value: &mut CellValue,
        _column_config: &mut ColumnConfig,
    ) -> Option<Response> {
        let mut n: f64 = cell_value.0.parse().unwrap_or(0.0);
        let res = ui.add(egui::DragValue::new(&mut n).speed(0.1));
        if res.changed() {
            cell_value.0 = n.to_string();
        }
        Some(res)
    }
}

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

pub struct SelectEditor;
impl ColumnTypeEditor for SelectEditor {
    fn show(
        &self,
        ui: &mut Ui,
        cell_value: &mut CellValue,
        column_config: &mut ColumnConfig,
    ) -> Option<Response> {
        let text = if cell_value.0.is_empty() { "Select...".to_string() } else { cell_value.0.clone() };
        let popup_id = ui.make_persistent_id("select_editor_popup");
        
        // We show a label as a placeholder in the cell.
        let placeholder_res = ui.selectable_label(false, text);
        
        // Check if the popup was open in the previous frame
        let was_open = egui::Popup::is_id_open(ui.ctx(), popup_id);

        // Force the popup to open immediately.
        if !was_open {
            egui::Popup::open_id(ui.ctx(), popup_id);
        }

        let mut response = placeholder_res.clone();

        egui::popup_below_widget(ui, popup_id, &placeholder_res, egui::PopupCloseBehavior::CloseOnClickOutside, |ui| {
            ui.set_min_width(150.0);
            
            let text_edit_res = ui.text_edit_singleline(&mut cell_value.0);
            if text_edit_res.changed() {
                response.mark_changed();
            }

            // If it was NOT open in the previous frame, but is open now (it is, since we are inside the popup),
            // it means it was just opened.
            if !was_open {
                text_edit_res.request_focus();
            }

            ui.separator();

            if let Some(allowed_values) = &column_config.allowed_values {
                for value in allowed_values {
                    if ui.selectable_label(&cell_value.0 == value, value).clicked() {
                        cell_value.0 = value.clone();
                        response.mark_changed();
                        ui.close();
                    }
                }
            }
        });

        // Update allowed_values only when the popup closes or if something changed.
        // If it was open but now it's closed, it means it just closed.
        let is_open = egui::Popup::is_id_open(ui.ctx(), popup_id);
        if was_open && !is_open {
            if !cell_value.0.is_empty() {
                let allowed = column_config.allowed_values.get_or_insert_with(Vec::new);
                if !allowed.contains(&cell_value.0) {
                    allowed.push(cell_value.0.clone());
                    response.mark_changed();
                }
            }
        }

        Some(response)
    }
}

pub struct MultiSelectEditor;
impl ColumnTypeEditor for MultiSelectEditor {
    fn show(
        &self,
        ui: &mut Ui,
        cell_value: &mut CellValue,
        _column_config: &mut ColumnConfig,
    ) -> Option<Response> {
        Some(ui.text_edit_singleline(&mut cell_value.0))
    }
}
