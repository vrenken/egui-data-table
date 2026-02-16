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
    #[serde(default)]
    pub order: usize,
    #[serde(default)]
    pub width: Option<f32>,
    #[serde(default)]
    pub allowed_values: Option<Vec<AllowedValue>>,
}