use egui::{Response, Ui, Color32};
use crate::data::*;

fn get_random_gentle_color() -> [u8; 3] {
    let h = fastrand::f32();
    let s = 0.5; // gentle saturation
    let l = 0.8; // gentle lightness
    
    let color = egui::ecolor::Hsva::new(h, s, l, 1.0);
    let rgb = Color32::from(color);
    [rgb.r(), rgb.g(), rgb.b()]
}

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
        let mut color = Color32::TRANSPARENT;

        if let Some(allowed) = &column_config.allowed_values {
            if let Some(av) = allowed.iter().find(|av| av.value == cell_value.0) {
                color = Color32::from_rgb(av.color[0], av.color[1], av.color[2]);
            }
        }

        let popup_id = ui.make_persistent_id("select_editor_popup");
        
        // We show a label as a placeholder in the cell.
        let placeholder_res = if color != Color32::TRANSPARENT {
            ui.scope(|ui| {
                ui.visuals_mut().widgets.inactive.weak_bg_fill = color;
                ui.visuals_mut().widgets.hovered.weak_bg_fill = color;
                ui.visuals_mut().widgets.active.weak_bg_fill = color;
                ui.button(text)
            }).inner
        } else {
            ui.selectable_label(false, text)
        };
        
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
                ui.ctx().request_repaint(); // Ensure it updates and eventually saves
            }
            if text_edit_res.lost_focus() || ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                ui.close();
            }

            // If it was NOT open in the previous frame, but is open now (it is, since we are inside the popup),
            // it means it was just opened.
            if !was_open {
                text_edit_res.request_focus();
            }

            ui.separator();

            if let Some(allowed_values) = &column_config.allowed_values {
                for av in allowed_values {
                    let av_color = Color32::from_rgb(av.color[0], av.color[1], av.color[2]);
                    let clicked = ui.scope(|ui| {
                        ui.visuals_mut().widgets.inactive.weak_bg_fill = av_color;
                        ui.visuals_mut().widgets.hovered.weak_bg_fill = av_color;
                        ui.visuals_mut().widgets.active.weak_bg_fill = av_color;
                        ui.selectable_label(cell_value.0 == av.value, &av.value)
                    }).inner.clicked();

                    if clicked {
                        cell_value.0 = av.value.clone();
                        response.mark_changed();
                        ui.ctx().request_repaint(); // Ensure it updates and eventually saves
                        ui.close();
                    }
                }
            }
        });

        // Update allowed_values and cell_values only when the popup closes or if something changed.
        // If it was open but now it's closed, it means it just closed.
        let is_open = egui::Popup::is_id_open(ui.ctx(), popup_id);
        if was_open && !is_open {
            if !cell_value.0.is_empty() {
                // 1. Update allowed_values in column_config
                let allowed = column_config.allowed_values.get_or_insert_with(Vec::new);
                if !allowed.iter().any(|av| av.value == cell_value.0) {
                    allowed.push(AllowedValue {
                        value: cell_value.0.clone(),
                        color: get_random_gentle_color(),
                    });
                    response.mark_changed();
                    ui.ctx().request_repaint(); // Ensure it updates and eventually saves
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

pub struct RelationEditor;

impl ColumnTypeEditor for RelationEditor {
    fn show(
        &self,
        ui: &mut Ui,
        cell_value: &mut CellValue,
        _column_config: &mut ColumnConfig,
    ) -> Option<Response> {
        ui.horizontal(|ui| {
            ui.label(egui_material_icons::icons::ICON_NORTH_EAST);
            ui.text_edit_singleline(&mut cell_value.0)
        })
        .inner
        .into()
    }
}
