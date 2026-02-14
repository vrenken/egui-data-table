use std::iter::repeat_with;
use crate::data::{Gender, Grade, Row, CellValue};

pub fn generate_random_data(count: usize) -> Vec<Row> {
    let mut rng = fastrand::Rng::new();
    let mut name_gen = names::Generator::with_naming(names::Name::Numbered);

    repeat_with(move || {
        Row {
            cells: vec![
                CellValue::String(name_gen.next().unwrap()),
                CellValue::Int(rng.i32(4..31)),
                CellValue::Gender(match rng.i32(0..=2) {
                    0 => None,
                    1 => Some(Gender::Male),
                    2 => Some(Gender::Female),
                    _ => unreachable!(),
                }),
                CellValue::Bool(rng.bool()),
                CellValue::Grade(rng.i32(0..=5).try_into().unwrap_or(Grade::F)),
                CellValue::Bool(false),
            ]
        }
    })
    .take(count)
    .collect()
}
