#[derive(Debug, Clone, PartialEq)]
pub enum CellValue {
    String(String),
    Int(i32),
    Float(f64),
    Bool(bool),
}

#[derive(Debug, Clone)]
pub struct Row {
    pub cells: Vec<CellValue>,
}