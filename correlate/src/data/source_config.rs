use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use crate::data::ColumnConfig;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SourceConfig {
    pub sheets: Vec<SheetConfig>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SheetConfig {
    pub name: String,
    pub column_configs: Vec<ColumnConfig>,
    pub sort_config: Option<SortConfig>,
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

    pub fn get_companion_path<P: AsRef<Path>>(path: P) -> PathBuf {
        let mut p = path.as_ref().to_path_buf();
        let ext = p.extension().and_then(|e| e.to_str()).unwrap_or("");
        let new_ext = if ext.is_empty() {
            "correlate".to_string()
        } else {
            format!("{}.correlate", ext)
        };
        p.set_extension(new_ext);
        p
    }
}
