use crate::egui_data_table::*;

pub struct Clipboard<R> {
    pub slab: Box<[R]>,

    /// The first tuple element `VisRowPos` is offset from the top-left corner of the
    /// selection.
    pub pastes: Box<[(VisRowOffset, ColumnIdx, RowSlabIndex)]>,
}

impl<R> Clipboard<R> {
    pub fn sort(&mut self) {
        self.pastes
            .sort_by(|(a_row, a_col, ..), (b_row, b_col, ..)| {
                a_row.0.cmp(&b_row.0).then(a_col.0.cmp(&b_col.0))
            })
    }
}