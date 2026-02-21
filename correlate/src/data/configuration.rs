use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::data::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Configuration {
    #[serde(skip)]
    pub path: std::path::PathBuf,

    pub data_sources: Vec<String>,
    pub selected_index: Option<usize>,
    pub projects: Option<Vec<ProjectConfiguration>>
}

impl Configuration {
    pub fn new<P: AsRef<Path>>(
        source_path: P,
    ) -> Self {
        Self {
            path: source_path.as_ref().to_path_buf(),
            data_sources: vec![],
            selected_index: Some(0),
            projects: Some(vec![])
        }
    }
}

impl Configuration {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let path_ref = path.as_ref();
        if !path_ref.exists() {
            let config = Self::new(path_ref);
            config.save()?;
            return Ok(config);
        }

        let content = fs::read_to_string(path_ref).map_err(|e| e.to_string())?;
        let mut config: Configuration = serde_json::from_str(&content).map_err(|e| e.to_string())?;
        config.path = path_ref.to_path_buf();
        Ok(config)
    }

    pub fn save(&self) -> Result<(), String> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let content = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        fs::write(self.path.as_path(), content).map_err(|e| e.to_string())?;
        Ok(())
    }
}