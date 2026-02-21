use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(skip)]
    pub path: std::path::PathBuf,

    pub data_sources: Vec<String>,
    pub selected_index: Option<usize>,
}

impl Config {
    pub fn new<P: AsRef<Path>>(
        source_path: P,
    ) -> Self {
        Self {
            path: source_path.as_ref().to_path_buf(),
            data_sources: vec![],
            selected_index: Some(0),
        }
    }
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        if !path.as_ref().exists() {
            let config = Self::new(path);
            config.save()?;
            return Ok(config);
        }

        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let config = serde_json::from_str(&content).map_err(|e| e.to_string())?;
        Ok(config)
    }

    pub fn save(&self) -> Result<(), String> {
        let content = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        fs::write(self.path.as_path(), content).map_err(|e| e.to_string())?;
        Ok(())
    }
}
