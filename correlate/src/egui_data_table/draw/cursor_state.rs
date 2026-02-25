use crate::egui_data_table::{RowIdx, VisColumnPos, VisSelection};

pub enum CursorState<R> {
    Select(Vec<VisSelection>),
    Edit {
        next_focus: bool,
        last_focus: VisColumnPos,
        row: RowIdx,
        edition: R,
    },
}