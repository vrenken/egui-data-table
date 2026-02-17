use egui::{Response, Ui};
use crate::data::*;
use super::ColumnTypeEditor;

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
            
            // Try to parse the current value as a Relation
            let mut relation = cell_value.0.parse::<Relation>().unwrap_or_else(|_| Relation::new("", "", ""));
            
            let mut changed = false;
            ui.label("Source:");
            if ui.text_edit_singleline(&mut relation.source).changed() { changed = true; }
            ui.label("Key:");
            if ui.text_edit_singleline(&mut relation.key).changed() { changed = true; }
            ui.label("Value:");
            if ui.text_edit_singleline(&mut relation.value).changed() { changed = true; }

            if changed {
                cell_value.0 = relation.to_string();
            }
            
            ui.text_edit_singleline(&mut cell_value.0)
        })
        .inner
        .into()
    }
}
