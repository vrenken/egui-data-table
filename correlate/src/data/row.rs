use crate::data::{Gender, Grade};

#[derive(Debug, Clone, PartialEq)]
pub enum CellValue {
    String(String),
    Int(i32),
    Gender(Option<Gender>),
    Bool(bool),
    Grade(Grade),
}

#[derive(Debug, Clone)]
pub struct Row {
    pub cells: Vec<CellValue>,
}

impl Row {
    pub fn new(cells: Vec<CellValue>) -> Self {
        Self { cells }
    }
}

impl Default for Row {
    fn default() -> Self {
        Row {
            cells: vec![
                CellValue::String("".to_string()),
                CellValue::Int(0),
                CellValue::Gender(None),
                CellValue::Bool(false),
                CellValue::Grade(Grade::F),
                CellValue::Bool(false),
            ]
        }
    }
}