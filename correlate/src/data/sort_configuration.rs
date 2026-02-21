use serde::{Deserialize, Serialize};
use crate::data::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SortConfiguration {
    pub column_name: String,
    pub is_ascending: bool,
}

