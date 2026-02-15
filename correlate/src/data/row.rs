#[derive(Debug, Clone, PartialEq)]
pub enum CellValue {
    String(String),
    Number(f64),
    DateTime(String),
    Bool(bool),
    Select(Option<String>),
    MultiSelect(Vec<String>),
}

#[derive(Debug, Clone)]
pub struct Row {
    pub cells: Vec<CellValue>,
}