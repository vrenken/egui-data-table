use crate::data::*;

pub trait SheetLoader {
    fn load(&self, path: String) -> Result<Vec<DataSheet>, String>;
}

#[derive(Clone)]
pub struct DataSheet {
    pub name: String,
    pub custom_name: Option<String>,
    pub display_name: Option<String>,
    pub icon: &'static str,
    pub column_configs: Vec<ColumnConfig>,
    pub table: egui_data_table::DataTable<Row>,
}