#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColumnType {
    String,
    Int,
    Gender,
    Bool,
    Grade,
}

#[derive(Debug, Clone)]
pub struct ColumnConfig {
    pub name: String,
    pub column_type: ColumnType,
    pub is_sortable: bool,
}

pub fn get_default_column_configs() -> Vec<ColumnConfig> {
    vec![
        ColumnConfig { name: "Name (Click to sort)".to_string(), column_type: ColumnType::String, is_sortable: true },
        ColumnConfig { name: "Age".to_string(), column_type: ColumnType::Int, is_sortable: true },
        ColumnConfig { name: "Gender".to_string(), column_type: ColumnType::Gender, is_sortable: true },
        ColumnConfig { name: "Is Student (Not sortable)".to_string(), column_type: ColumnType::Bool, is_sortable: false },
        ColumnConfig { name: "Grade".to_string(), column_type: ColumnType::Grade, is_sortable: true },
        ColumnConfig { name: "Row locked".to_string(), column_type: ColumnType::Bool, is_sortable: true },
    ]
}
