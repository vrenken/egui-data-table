use egui::{Popup, Response, Ui};
use crate::data::*;
use crate::view::*;

pub struct RelationEditor;

impl ColumnTypeEditor for RelationEditor {
    fn show(
        &self,
        ui: &mut Ui,
        cell_value: &mut CellValue,
        column_config: &mut ColumnConfiguration,
        view_model: &mut RootViewModel
    ) -> Option<Response> {
        // Determine the related sheet from ColumnConfig.related_source (format: "Source > Sheet")
        let related = column_config.related_source.clone().unwrap_or_default();
        let mut parts = related.splitn(2, " > ");
        let src_part = parts.next().unwrap_or("");
        let sheet_part = parts.next().unwrap_or("");

        // Resolve the related DataSheet
        let mut rel_sheet: Option<&DataSheet> = None;
        let mut resolved_src_name = String::new();
        if !src_part.is_empty() && !sheet_part.is_empty() {
            for ds in &view_model.data_sources {
                let display_source = ds.name.as_ref().unwrap_or(&ds.path);
                if display_source == src_part {
                    for sheet in &ds.sheets {
                        let display_sheet = sheet.display_name.as_ref().unwrap_or(&sheet.name);
                        if display_sheet == sheet_part {
                            rel_sheet = Some(sheet);
                            resolved_src_name = format!("{} > {}", display_source, display_sheet);
                            break;
                        }
                    }
                }
                if rel_sheet.is_some() { break; }
            }
        }

        // Fallback UI if no related sheet is configured
        if rel_sheet.is_none() {
            return Some(ui.selectable_label(false, "No related source configured"));
        }
        let rel_sheet = rel_sheet.unwrap();

        // Find key and name columns in the related sheet
        let key_col_idx = rel_sheet.column_configs.iter().position(|c| c.is_key).unwrap_or(0);
        let name_col_idx = ColumnConfiguration::find_name_column_index(&rel_sheet.column_configs);

        // Prepare the current display text
        let mut current_key = cell_value.0.clone();
        if let Ok(relation) = cell_value.0.parse::<Relation>() {
            current_key = relation.key;
        }

        let mut current_display = String::new();
        if !current_key.is_empty() {
            if let Some(row) = rel_sheet.table.iter().find(|r| r.cells.get(key_col_idx).map(|c| &c.0) == Some(&current_key)) {
                if let Some(name_cell) = row.cells.get(name_col_idx) {
                    current_display = name_cell.0.clone();
                }
            }
        }
        let placeholder = if current_display.is_empty() {
            if current_key.is_empty() { "Select...".to_string() } else { current_key.clone() }
        } else {
            current_display
        };

        // Prepare options for the popup (to avoid borrowing view_model inside the closure)
        let query_initial = current_key.to_lowercase();
        let mut options = Vec::new();
        for row in rel_sheet.table.iter() {
            let key = row.cells.get(key_col_idx).map(|c| c.0.clone()).unwrap_or_default();
            if key.is_empty() { continue; }
            let display = row.cells.get(name_col_idx).map(|c| c.0.clone()).unwrap_or_else(|| key.clone());
            if !query_initial.is_empty() && !display.to_lowercase().contains(&query_initial) {
                // We'll filter again inside the closure because query_buffer can change,
                // but we need to collect all possible candidates or just all rows.
                // Actually, let's collect ALL rows with key/display pairs to be safe, 
                // or just re-resolve inside the closure if we must.
                // Collecting all for now as it's simpler.
            }
            options.push((key, display));
        }

        let popup_id = ui.make_persistent_id("relation_editor_popup");
        let placeholder_res = ui.selectable_label(false, placeholder);

        // Check if the popup was open in the previous frame
        let was_open = Popup::is_id_open(ui.ctx(), popup_id);

        // Force the popup to open immediately.
        if !was_open {
            Popup::open_id(ui.ctx(), popup_id);
        }
        let mut response = placeholder_res.clone();

        egui::popup_below_widget(ui, popup_id, &placeholder_res, egui::PopupCloseBehavior::CloseOnClickOutside, |ui| {
            ui.set_min_width(220.0);

            // Use the current key for the text edit query, but we need to handle the fact that it might be a serialized relation
            let mut query_buffer = current_key.clone();
            let text_edit_res = ui.text_edit_singleline(&mut query_buffer);
            
            // Ensure the text box gets focus when the popup is first opened.
            if !was_open {
                text_edit_res.request_focus();
            }

            if text_edit_res.changed() {
                cell_value.0 = query_buffer.clone();
                response.mark_changed();
                ui.ctx().request_repaint();
            }
            if text_edit_res.lost_focus() || ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                view_model.save_datasource_configuration();
                Popup::close_id(ui.ctx(), popup_id);
            }

            ui.separator();

            // Build and filter options from collected options
            let query = query_buffer.to_lowercase();
            for (key, display) in options {
                if !query.is_empty() && !display.to_lowercase().contains(&query) { continue; }

                let clicked = ui.selectable_label(current_key == key, &display).clicked();
                if clicked {
                    let relation = Relation::new(resolved_src_name.clone(), key, display);
                    cell_value.0 = relation.to_string();
                    response.mark_changed();
                    ui.ctx().request_repaint();
                    view_model.save_datasource_configuration();
                    Popup::close_id(ui.ctx(), popup_id);
                }
            }
        });

        let is_open = Popup::is_id_open(ui.ctx(), popup_id);
        if was_open && !is_open {
            if !cell_value.0.is_empty() {
                    response.mark_changed();
                    ui.ctx().request_repaint(); // Ensure it updates and eventually saves
                view_model.save_datasource_configuration();
                Popup::close_id(ui.ctx(), popup_id);
            }
        }
        Some(response)
    }
}
