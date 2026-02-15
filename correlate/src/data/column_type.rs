use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColumnType {
    Text,
    Number,
    DateTime,
    Bool,
}

impl ColumnType {
    pub fn icon(&self) -> &'static str {
        match self {
            ColumnType::Text => egui_material_icons::icons::ICON_SUBJECT,
            ColumnType::Number => egui_material_icons::icons::ICON_TAG,
            ColumnType::DateTime => egui_material_icons::icons::ICON_CALENDAR_CLOCK,
            ColumnType::Bool => egui_material_icons::icons::ICON_CHECK_BOX,
        }
    }
}