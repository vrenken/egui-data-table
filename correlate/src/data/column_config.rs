use serde::{Deserialize, Serialize};
use crate::data::ColumnType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnConfig {
    pub name: String,
    pub column_type: ColumnType,
    pub is_sortable: bool,
    #[serde(default)]
    pub is_key: bool,
    #[serde(default)]
    pub is_name: bool,
    #[serde(default)]
    pub is_virtual: bool,
    #[serde(default)]
    pub width: Option<f32>,
}