use serde::{Deserialize, Serialize};
use crate::data::*;

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

