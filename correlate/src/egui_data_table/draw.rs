use egui::{Align, Color32, Vec2b};

use egui::scroll_area::ScrollBarVisibility;

pub(crate) mod state;
mod tsv;
pub(crate) mod ui_state;
mod clipboard;
pub(crate) mod command;
mod persist_data;
mod cursor_state;
/* -------------------------------------------- Style ------------------------------------------- */

/// Style configuration for the table.
// TODO: Implement more style configurations.
#[derive(Default, Debug, Clone, Copy)]
#[non_exhaustive]
pub struct Style {
    /// Background color override for selection. Default uses `visuals.selection.bg_fill`.
    pub bg_selected_cell: Option<Color32>,

    /// Background color override for selected cell. Default uses `visuals.selection.bg_fill`.
    pub bg_selected_highlight_cell: Option<Color32>,

    /// Foreground color override for selected cell. Default uses `visuals.strong_text_colors`.
    pub fg_selected_highlight_cell: Option<Color32>,

    /// Foreground color for cells that are going to be selected when the mouse is dropped.
    pub fg_drag_selection: Option<Color32>,

    /* ·························································································· */
    /// Maximum number of undo history. This is applied when the actual action is performed.
    ///
    /// Setting value '0' results in kinda appropriate default value.
    pub max_undo_history: usize,

    /// If specify this as [`None`], the heterogeneous row height will be used.
    pub table_row_height: Option<f32>,

    /// When enabled, single click on a cell will start editing mode. Default is `false` where
    /// double action(click 1: select, click 2: edit) is required.
    pub single_click_edit_mode: bool,

    /// How to align cell contents. Default is left-aligned.
    pub cell_align: Align,

    /// Color to use for the stroke above/below focused row.
    /// If `None`, defaults to a darkened `warn_fg_color`.
    pub focused_row_stroke: Option<Color32>,

    /// See [`ScrollArea::auto_shrink`] for details.
    pub auto_shrink: Vec2b,

    /// See ['ScrollArea::ScrollBarVisibility`] for details.
    pub scroll_bar_visibility: ScrollBarVisibility,
}

/* ------------------------------------------ Rendering ----------------------------------------- */

/* ------------------------------------------- Translations ------------------------------------- */

pub trait Translator {

    /// Translates a given key into its corresponding string representation.
    ///
    /// If the translation key is unknown, return the key as a [`String`]
    fn translate(&self, key: &str) -> String;
}

#[derive(Default)]
pub struct EnglishTranslator {}

impl Translator for EnglishTranslator {
    fn translate(&self, key: &str) -> String {
        match key {
            // cell context menu
            "context-menu-selection-copy" => "Selection: Copy",
            "context-menu-selection-cut" => "Selection: Cut",
            "context-menu-selection-clear" => "Selection: Clear",
            "context-menu-selection-fill" => "Selection: Fill",
            "context-menu-clipboard-paste" => "Clipboard: Paste",
            "context-menu-clipboard-insert" => "Clipboard: Insert",
            "context-menu-row-duplicate" => "Row: Duplicate",
            "context-menu-row-delete" => "Row: Delete",
            "context-menu-undo" => "Undo",
            "context-menu-redo" => "Redo",

            // column header context menu
            "context-menu-hide" => "Hide",
            "context-menu-hidden" => "Hidden",
            "context-menu-clear-sort" => "Clear sort",
            _ => key,
        }.to_string()
    }
}


