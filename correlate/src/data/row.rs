#[derive(Debug, Clone, PartialEq)]
pub struct CellValue(pub String);

impl CellValue {
    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl From<&str> for CellValue {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<String> for CellValue {
    fn from(s: String) -> Self {
        Self(s)
    }
}

#[derive(Debug, Clone)]
pub struct Row {
    pub cells: Vec<CellValue>,
}