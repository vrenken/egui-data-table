use crate::egui_data_table::{CellWriteContext, ColumnIdx, IsAscending, RowIdx, RowSlabIndex, VisColumnPos, VisSelection};

/// NOTE: `Cc` prefix stands for cache command which won't be stored in the undo/redo queue, since they
/// are not called from the `cmd_apply` method.
pub enum Command<R> {
    CcHideColumn(ColumnIdx),
    CcShowColumn {
        what: ColumnIdx,
        at: VisColumnPos,
    },
    CcReorderColumn {
        from: VisColumnPos,
        to: VisColumnPos,
    },

    SetColumnSort(Vec<(ColumnIdx, IsAscending)>),
    SetVisibleColumns(Vec<ColumnIdx>),

    CcSetSelection(Vec<VisSelection>), // Cache - Set Selection

    SetRowValue(RowIdx, Box<R>),
    CcSetCells {
        slab: Box<[R]>,
        values: Box<[(RowIdx, ColumnIdx, RowSlabIndex)]>,
        context: CellWriteContext,
    },
    SetCells {
        slab: Box<[R]>,
        values: Box<[(RowIdx, ColumnIdx, RowSlabIndex)]>,
    },

    InsertRows(RowIdx, Box<[R]>),
    AddColumn(usize),
    MoveColumn(usize, usize),
    RenameCommitted(crate::egui_data_table::viewer::RenameTarget, String),
    RequestSave,
    RemoveRow(Vec<RowIdx>),
    RemoveColumn(usize),

    CcEditStart(RowIdx, VisColumnPos, Box<R>),
    CcCancelEdit,
    CcCommitEdit,

    CcUpdateSystemClipboard(String),
}