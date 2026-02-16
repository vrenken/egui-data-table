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
        if let CellValue::String(s) = cell_value {
            Some(egui::TextEdit::multiline(s)
                .desired_rows(1)
                .code_editor()
                .show(ui)
                .response)
        } else {
            None
        }
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
        if let CellValue::Number(n) = cell_value {
            Some(ui.add(egui::DragValue::new(n).speed(0.1)))
        } else {
            None
        }
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
        if let CellValue::DateTime(dt) = cell_value {
            Some(egui::TextEdit::singleline(dt)
                .show(ui)
                .response)
        } else {
            None
        }
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
        if let CellValue::Bool(b) = cell_value {
            Some(ui.checkbox(b, ""))
        } else {
            None
        }
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
        if let CellValue::Select(s) = cell_value {
            let popup_id = ui.id().with("popup");
            let text = s.clone().unwrap_or_default();
            let mut response = ui.add_sized(ui.available_size(), egui::Label::new(text).sense(egui::Sense::click()));

            if !egui::Popup::is_id_open(ui.ctx(), popup_id) {
                egui::Popup::open_id(ui.ctx(), popup_id);
            }

            let inner = egui::Popup::from_response(&response)
                .id(popup_id)
                .show(|ui| {
                    ui.set_min_width(150.0);
                    let mut edit_text = s.clone().unwrap_or_default();
                    if ui.text_edit_singleline(&mut edit_text).changed() {
                        *s = if edit_text.is_empty() { None } else { Some(edit_text) };
                        response.mark_changed();
                    }

                    ui.separator();

                    if let Some(allowed_values) = &column_config.allowed_values {
                        for value in allowed_values {
                            if ui.selectable_label(s.as_deref() == Some(value), value).clicked() {
                                *s = Some(value.clone());
                                response.mark_changed();
                                ui.close();
                            }
                        }
                    }
                });

            if let Some(inner) = inner {
                response = response.union(inner.response);
            }

            if response.changed() || !egui::Popup::is_id_open(ui.ctx(), popup_id) {
                if let Some(val) = s.as_ref() {
                    let allowed = column_config.allowed_values.get_or_insert_with(Vec::new);
                    if !allowed.contains(val) {
                        allowed.push(val.clone());
                    }
                }
            }

            Some(response)
        } else {
            None
        }
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
        if let CellValue::MultiSelect(v) = cell_value {
            let mut text = v.join(", ");
            let res = ui.text_edit_singleline(&mut text);
            if res.changed() {
                *v = text.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
            }
            Some(res)
        } else {
            None
        }
    }
}
