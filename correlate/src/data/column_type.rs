use serde::{Deserialize, Serialize};
use egui::{Response, Ui};
use crate::data::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColumnType {
    Text,
    Number,
    DateTime,
    Bool,
    Select,
    MultiSelect,
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
        }
    }

    pub fn show_editor(
        &self,
        ui: &mut Ui,
        cell_value: &mut CellValue,
        column_config: &mut ColumnConfig,
    ) -> Option<Response> {
        self.editor().show(ui, cell_value, column_config)
    }

    pub fn default_value(&self) -> CellValue {
        CellValue("".to_string())
    }
}