// column indices
// columns can easily be reordered simply by changing the values of these indices.
pub const NAME: usize = 0;
pub const AGE: usize = 1;
pub const GENDER: usize = 2;
pub const IS_STUDENT: usize = 3;
pub const GRADE: usize = 4;
pub const ROW_LOCKED: usize = 5;

/// count of columns
pub const COLUMN_COUNT: usize = 6;

pub const COLUMN_NAMES: [&str; COLUMN_COUNT] = [
    "Name (Click to sort)",
    "Age",
    "Gender",
    "Is Student (Not sortable)",
    "Grade",
    "Row locked",
];
