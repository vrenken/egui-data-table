use serde::{Deserialize, Serialize};
use crate::data::ColumnType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnConfig {
    pub name: String,
    #[serde(default, rename = "displayName")]
    pub display_name: Option<String>,
    pub column_type: ColumnType,
    pub is_sortable: bool,
    #[serde(default)]
    pub is_key: bool,
    #[serde(default)]
    pub is_name: bool,
    #[serde(default)]
    pub is_virtual: bool,
    #[serde(default)]
    pub order: usize,
    #[serde(default)]
    pub width: Option<f32>,
}