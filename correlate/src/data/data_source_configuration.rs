use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::data::*;

pub struct DataSourceConfiguration {
    pub sheets: Vec<DataSheetConfiguration>,
}

impl DataSourceConfiguration {
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SourceConfig {
    #[serde(skip)]
    pub path: std::path::PathBuf,
    #[serde(default)]
    pub name: Option<String>,
    pub sheets: Vec<DataSheetConfiguration>,
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
        sheets: Vec<DataSheetConfiguration>,
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