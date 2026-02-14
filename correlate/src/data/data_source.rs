use std::iter::repeat_with;
use crate::data::{Gender, Grade, Row, CellValue, ColumnConfig, ColumnType};

pub fn get_rows(count: usize, configs: &[ColumnConfig]) -> Vec<Row> {
    let mut rng = fastrand::Rng::new();
    let mut name_gen = names::Generator::with_naming(names::Name::Numbered);

    let configs = configs.to_vec();

    repeat_with(move || {
        let mut cells = Vec::with_capacity(configs.len());
        for config in &configs {
            let cell = match config.column_type {
                ColumnType::String => CellValue::String(name_gen.next().unwrap()),
                ColumnType::Int => CellValue::Int(rng.i32(4..31)),
                ColumnType::Gender => CellValue::Gender(match rng.i32(0..=2) {
                    0 => None,
                    1 => Some(Gender::Male),
                    2 => Some(Gender::Female),
                    _ => unreachable!(),
                }),
                ColumnType::Bool => CellValue::Bool(rng.bool()),
                ColumnType::Grade => CellValue::Grade(rng.i32(0..=5).try_into().unwrap_or(Grade::F)),
            };
            cells.push(cell);
        }
        Row { cells }
    })
    .take(count)
    .collect()
}
