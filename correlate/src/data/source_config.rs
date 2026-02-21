use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use crate::data::ColumnConfig;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SourceConfig {
    #[serde(skip)]
    pub path: std::path::PathBuf,
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
    fn calculate_path<P: AsRef<Path>>(source_path: P) -> std::path::PathBuf {
        let mut p = source_path.as_ref().to_path_buf();
        let ext = p.extension().and_then(|e| e.to_str()).unwrap_or("");
        let new_ext = if ext.is_empty() {
            "correlate".to_string()
        } else {
            format!("{}.correlate", ext)
        };
        p.set_extension(new_ext);
        p
    }

    pub fn new<P: AsRef<Path>>(
        source_path: P,
        name: Option<String>,
        sheets: Vec<SheetConfig>,
    ) -> Self {
        Self {
            path: Self::calculate_path(source_path),
            name,
            sheets,
        }
    }

    pub fn load<P: AsRef<Path>>(source_path: P) -> Self {
        let path_buf = Self::calculate_path(&source_path);
        match fs::read_to_string(&path_buf) {
            Ok(content) => {
                match serde_json::from_str::<Self>(&content) {
                    Ok(mut config) => {
                        config.path = path_buf;
                        config
                    }
                    Err(e) => {
                        log::error!("Failed to parse config at {:?}: {}", path_buf, e);
                        Self::new(source_path, None, Vec::new())
                    }
                }
            }
            Err(_) => {
                // File likely does not exist, return a new default config
                Self::new(source_path, None, Vec::new())
            }
        }
    }

    pub fn save(&self) -> Result<(), String> {
        let content = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        fs::write(&self.path, content).map_err(|e| e.to_string())?;
        Ok(())
    }
}
