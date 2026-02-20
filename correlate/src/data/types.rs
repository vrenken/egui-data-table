use crate::data::{ColumnConfig, Row};

pub trait Sheet {
    fn name(&self) -> &str;
    fn custom_name(&self) -> Option<&str>;
    fn display_name(&self) -> Option<&str>;
    fn column_configs(&self) -> &[ColumnConfig];
    fn rows(self: Box<Self>) -> Vec<Row>;
    fn cloned_rows(&self) -> Vec<Row>;
}

#[derive(Clone)]
pub struct DataSheet {
    pub name: String,
    pub display_name: Option<String>,
    pub column_configs: Vec<crate::data::ColumnConfig>,
    pub table: egui_data_table::DataTable<Row>,
}

#[derive(Clone)]
pub struct DataSource {
    pub path: String,
    pub name: Option<String>,
    pub sheets: Vec<DataSheet>,
    pub selected_sheet_index: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RenamingTarget {
    DataSource(usize),
    Sheet(usize, usize),
    Row(usize),
    Column(usize),
}
