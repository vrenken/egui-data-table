#[derive(Debug, Clone, PartialEq)]
pub struct CellValue(pub String);

impl CellValue {
    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Row {
    pub cells: Vec<CellValue>,
}