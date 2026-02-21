use serde::{Deserialize, Serialize};
use crate::data::*;

pub struct DataSheetConfiguration {

}

impl DataSheetConfiguration {

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