use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CellValueConfiguration {
    pub key: String,
    pub column_name: String,
    pub value: String,
}