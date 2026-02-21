use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectConfiguration {
    pub name: String,
    pub data_sources: Vec<String>,
}
