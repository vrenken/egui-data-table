use serde::{Deserialize, Serialize};

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
}