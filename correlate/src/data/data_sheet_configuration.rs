use serde::{Deserialize, Serialize};
use crate::data::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DataSheetConfiguration {
    pub name: String,
    #[serde(default, rename = "display_name")]
    pub display_name: Option<String>,
    pub column_configs: Vec<ColumnConfiguration>,
    pub sort_config: Option<SortConfig>,
    #[serde(default)]
    pub cell_values: Vec<CellValueConfiguration>,
}

impl DataSheetConfiguration {

}