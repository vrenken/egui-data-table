use crate::data::{Gender, Grade};

#[derive(Debug, Clone)]
pub struct Row {
    pub name: String,
    pub age: i32,
    pub gender: Option<Gender>,
    pub is_student: bool,
    pub grade: Grade,
    pub row_locked: bool
}

impl Default for Row {
    fn default() -> Self {
        Row {
            name: "".to_string(),
            age: 0,
            gender: None,
            is_student: false,
            grade: Grade::F,
            row_locked: false
        }
    }
}