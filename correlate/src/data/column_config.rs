use crate::data::ColumnType;

#[derive(Debug, Clone)]
pub struct ColumnConfig {
    pub name: String,
    pub column_type: ColumnType,
    pub is_sortable: bool,
}