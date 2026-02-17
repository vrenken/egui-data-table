use serde::{Deserialize, Serialize};
use crate::data::ColumnType;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AllowedValue {
    pub value: String,
    pub color: [u8; 3],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnConfig {
    pub name: String,
    #[serde(default)]
    pub display_name: Option<String>,
    pub column_type: ColumnType,
    #[serde(default)]
    pub is_key: bool,
    #[serde(default)]
    pub is_name: bool,
    #[serde(default)]
    pub is_virtual: bool,
    #[serde(default = "default_true")]
    pub is_visible: bool,
    #[serde(default)]
    pub order: usize,
    #[serde(default)]
    pub width: Option<f32>,
    #[serde(default)]
    pub allowed_values: Option<Vec<AllowedValue>>,
    #[serde(default)]
    pub related_source: Option<String>,
}

fn default_true() -> bool {
    true
}

impl ColumnConfig {
    pub fn find_name_column_index(configs: &[ColumnConfig]) -> usize {
        configs.iter().position(|c| c.is_name)
            .or_else(|| configs.iter().position(|c| c.name.to_lowercase().contains("name")))
            .or_else(|| configs.iter().position(|c| c.column_type == ColumnType::Text))
            .unwrap_or(0)
    }
}