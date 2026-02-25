use crate::egui_data_table::{ColumnIdx, IsAscending};

#[cfg_attr(feature = "persistency", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Default)]
pub(crate) struct PersistData {
    /// Cached number of columns.
    pub num_columns: usize,

    /// Visible columns selected by the user.
    pub vis_cols: Vec<ColumnIdx>,

    /// Column sorting state.
    pub sort: Vec<(ColumnIdx, IsAscending)>,
}