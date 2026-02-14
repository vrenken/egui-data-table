use crate::data::column_config::ColumnConfig;
use crate::data::column_type::ColumnType;

pub fn get_default_column_configs() -> Vec<ColumnConfig> {
    vec![
        ColumnConfig { name: "Name (Click to sort)".to_string(), column_type: ColumnType::String, is_sortable: true, width: None },
        ColumnConfig { name: "Age".to_string(), column_type: ColumnType::Int, is_sortable: true, width: None },
        ColumnConfig { name: "Gender".to_string(), column_type: ColumnType::Gender, is_sortable: true, width: None },
        ColumnConfig { name: "Is Student (Not sortable)".to_string(), column_type: ColumnType::Bool, is_sortable: false, width: None },
        ColumnConfig { name: "Grade".to_string(), column_type: ColumnType::Grade, is_sortable: true, width: None },
        ColumnConfig { name: "Row locked".to_string(), column_type: ColumnType::Bool, is_sortable: true, width: None },
    ]
}
