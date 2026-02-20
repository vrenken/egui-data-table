use serde::{Deserialize, Serialize};
use egui::{Response, Ui};
use crate::data::*;
use crate::view::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColumnType {
    Text,
    Number,
    DateTime,
    Bool,
    Select,
    MultiSelect,
    Relation,
}

impl ColumnType {
    fn editor(&self) -> Box<dyn ColumnTypeEditor> {
        match self {
            ColumnType::Text => Box::new(TextEditor),
            ColumnType::Number => Box::new(NumberEditor),
            ColumnType::DateTime => Box::new(DateTimeEditor),
            ColumnType::Bool => Box::new(BoolEditor),
            ColumnType::Select => Box::new(SelectEditor),
            ColumnType::MultiSelect => Box::new(MultiSelectEditor),
            ColumnType::Relation => Box::new(RelationEditor),
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            ColumnType::Text => egui_material_icons::icons::ICON_SUBJECT,
            ColumnType::Number => egui_material_icons::icons::ICON_TAG,
            ColumnType::DateTime => egui_material_icons::icons::ICON_CALENDAR_CLOCK,
            ColumnType::Bool => egui_material_icons::icons::ICON_CHECK_BOX,
            ColumnType::Select => egui_material_icons::icons::ICON_ARROW_DROP_DOWN_CIRCLE,
            ColumnType::MultiSelect => egui_material_icons::icons::ICON_LIST,
            ColumnType::Relation => egui_material_icons::icons::ICON_NORTH_EAST,
        }
    }

    pub fn show_editor(
        &self,
        ui: &mut Ui,
        cell_value: &mut CellValue,
        column_config: &mut ColumnConfig,
        view_model: &mut RootViewModel

    ) -> Option<Response> {
        self.editor().show(ui, cell_value, column_config, view_model)
    }

    pub fn default_value(&self) -> CellValue {
        CellValue("".to_string())
    }

    pub fn load(
        &self,
        physical_value: Option<&str>,
        config: &ColumnConfig,
        row_key: Option<&str>,
        stored_values: Option<&[CellValueConfig]>
    ) -> CellValue {
        if config.is_virtual {
            let mut val = "".to_string();
            if let (Some(key), Some(stored)) = (row_key, stored_values) {
                if let Some(cv) = stored.iter().find(|cv| cv.key == key && cv.column_name == config.name) {
                    val = cv.value.clone();
                }
            }
            CellValue(val)
        } else {
            let value = physical_value.unwrap_or("");
            map_cell_value(value, *self)
        }
    }
}