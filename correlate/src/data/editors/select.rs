use egui::{Response, Ui, Color32, Popup};
use crate::data::*;
use super::{ColumnTypeEditor, get_random_gentle_color};

pub struct SelectEditor;
impl ColumnTypeEditor for SelectEditor {
    fn show(
        &self,
        ui: &mut Ui,
        cell_value: &mut CellValue,
        column_config: &mut ColumnConfig,
        _data_sources: &[DataSource],
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
            Popup::open_id(ui.ctx(), popup_id);
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
                // Case-insensitive filter of allowed values based on the current text input
                let query = cell_value.0.to_lowercase();
                for av in allowed_values.iter().filter(|av| query.is_empty() || av.value.to_lowercase().contains(&query)) {
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
                    //ui.close();
                    Popup::close_all(ui.ctx());//ui.ctx(), popup_id);
                }
            }
        }

        Some(response)
    }
}
