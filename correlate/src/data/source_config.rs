use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use crate::data::ColumnConfig;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SourceConfig {
    #[serde(default)]
    pub name: Option<String>,
    pub sheets: Vec<SheetConfig>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SheetConfig {
    pub name: String,
    #[serde(default, rename = "display_name")]
    pub display_name: Option<String>,
    pub column_configs: Vec<ColumnConfig>,
    pub sort_config: Option<SortConfig>,
    #[serde(default)]
    pub cell_values: Vec<CellValueConfig>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CellValueConfig {
    pub key: String,
    pub column_name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SortConfig {
    pub column_name: String,
    pub is_ascending: bool,
}

impl SourceConfig {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let config = serde_json::from_str(&content).map_err(|e| e.to_string())?;
        Ok(config)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
        let content = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        fs::write(path, content).map_err(|e| e.to_string())?;
        Ok(())
    }
}
