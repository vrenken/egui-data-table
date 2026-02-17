# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog],
and this project adheres to [Semantic Versioning].

## [Unreleased]

### Changed
- Relation editor now behaves like a select: it sources options from the configured `related_source`, and stores selections as serialized `Relation` URIs (`relation://{source}/{key}/{value}`).
- Relation cell visualization now shows only the human-readable name (`value`) instead of the full serialized URI, with graceful fallback for legacy/invalid strings.
- `ColumnTypeEditor` system was modularized into separate files under `correlate/src/data/editors/`, and the trait signature now includes `&[DataSource]` so editors can resolve cross-sheet data; the call sites were updated to wire data sources through the pipeline.
- Improved `Select` and `Relation` editors to correctly close the popup when pressing Enter or selecting an item.
- Fixed an issue where the text box in `Select` and `Relation` editors did not automatically gain focus when the popup was opened.

## [0.1.1] - 2026-02-17

### Added
- New `Relation` data type with format `relation://{source}/{key}/{value}`.
- Case-insensitive filtering for the allowed values list in the `Select` column editor.
- Compact serialization for `.correlate` files, reducing file size while maintaining readability.
- New `HeaderAction::MoveColumn` to handle column reordering at the library level with undo/redo support.
- Specific icons for Excel and CSV data sources in the hierarchy treeview.
- Ability to mark a column as the "name" column from the column header context menu. This column is marked with a 🏷️ icon.
- Ability to reorder columns from the column header menu using "Move Left" and "Move Right". Column order is now persisted to the `.correlate` file.
- Ability to rename a row by double-clicking its row header. The new name is persisted to the "name" column.
- New cell type for float values, with support for automatic inference from CSV and Excel files.
- New cell type for datetime values, with support for automatic inference from CSV and Excel files.
- New popup panel that is shown when a column header is clicked with the left mouse button.

### Fixed
- Fixed column data desynchronization when moving columns left or right.
- Resolved "Use as name" consistency issues where multiple columns could appear as the name column.
- Fixed `ComboBox` and type safety issues in `Relation` source selection.
- After the configuration is loaded, CSV data sources are now correctly loaded. Previously, only XLSX files were supported during initial load.
- Fixed an issue where renamed data sources and sheets were not persisted to the `.correlate` file.
- Fixed an issue where the column order was not correctly updated in the `.correlate` file after reordering.
- Fixed an issue where drag-and-drop column reordering did not synchronize with the underlying data and configuration.
- Fixed an issue where double-clicking a column header to rename it was overruled by a single click toggling the sort order.

### Changed
- Major architectural refactor:
    - Replaced string-based `CellValue` with a type-safe enum.
    - Introduced `ColumnBehavior` trait to encapsulate type-specific logic.
    - Centralized state mutations into a single reducer for consistency.
- Replaced the `Related source` combobox with a submenu containing checkboxes for better usability.
- Refined focus management and key handling in the column renaming field.
- The column rename functionality is now integrated directly into the column header context menu as a permanent textbox, replacing the previous "Rename" button and the double-click trigger in the header itself.
- Refactored context menus and event handling:
    - Moved column rename functionality to the column header context menu.
    - Moved column header and row context menus into dedicated modules (`central_panel_header_menu.rs`, `central_panel_row_context_menu.rs`).
    - Extracted the hierarchy panel context menu into `hierarchy_panel_context_menu.rs`.
    - Simplified central panel UI logic by moving event handling into a private `handle_viewer_requests` method.

## [0.1.0] - 2026-02-14

### Added
- Ability to add CSV files from the context menu.
- Ability to mark columns as keys from the cell and column header context menus.
- Ability to add virtual columns from the column header context menu, which are stored in the `.correlate` file and marked with a 🧪 icon.
- Ability to rename data sources and sheets in the treeview by double-clicking. Sheet names are stored as `display_name` in the `.correlate` file.

### Changed
- Removed default sample data sources ("Students" and "Random Data") and associated generator logic.
- Treeview now avoids adding child nodes for data sources with only one sheet.
### Fixed
- 
### Exposed
- 

### Prevented
- The root node ("Data Sources") in the hierarchy panel can no longer be collapsed.

## [Wishlist]

- [ ] Ability to work with multiple data sheets from Excel
- [x] Ability to work with CSV files
- [ ] Ability to add new data
- [x] Ability to mark columns as keys

<!-- Links -->
[keep a changelog]: https://keepachangelog.com/en/1.0.0/
[semantic versioning]: https://semver.org/spec/v2.0.0.html
