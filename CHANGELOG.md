# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog],
and this project adheres to [Semantic Versioning].

## [Unreleased]

### Added
- Ability to mark a column as the "name" column from the column header context menu. This column is marked with a 🏷️ icon.
- Ability to reorder columns from the column header menu using "Move Left" and "Move Right". Column order is now persisted to the `.correlate` file.
- Ability to rename a row by double-clicking its row header. The new name is persisted to the "name" column.
- New cell type for float values, with support for automatic inference from CSV and Excel files.

### Fixed
- Fixed an issue where renamed data sources and sheets were not persisted to the `.correlate` file.
- Fixed an issue where the column order was not correctly updated in the `.correlate` file after reordering.
- Fixed an issue where drag-and-drop column reordering did not synchronize with the underlying data and configuration.
- Fixed an issue where double-clicking a column header to rename it was overruled by a single click toggling the sort order.

### Changed
- The visible name of a column is now stored as `displayName` in the `.correlate` file, while keeping the internal `name` unchanged.
- Refactored `CorrelateApp` from `correlate/src/view/root_view.rs` into multiple modules in `correlate/src/view/app/` to improve code organization and maintainability.
- Split UI components into `menu_bar.rs`, `bottom_panel.rs`, `central_panel.rs`, and `hierarchy_panel.rs`.
- Moved data source management logic to `data_sources.rs` and shared types to `types.rs`.
- Extracted value inference and mapping logic from `csv.rs` and `excel.rs` into a new `value_mapping.rs` module.

## [0.1.0] - 2026-02-14

### Added
- Ability to add CSV files from the context menu.
- Ability to mark columns as keys from the cell and column header context menus.
- Ability to add virtual columns from the column header context menu, which are stored in the `.correlate` file and marked with a 🧪 icon.
- Ability to rename data sources and sheets in the treeview by double-clicking. Sheet names are stored as `displayName` in the `.correlate` file.

### Changed
- Removed default sample data sources ("Students" and "Random Data") and associated generator logic.
- Treeview now avoids adding child nodes for data sources with only one sheet.
### Fixed
- 
### Exposed
- 

### Prevented
- 

## [Wishlist]

- [ ] Ability to work with multiple data sheets from Excel
- [x] Ability to work with CSV files
- [ ] Ability to add new data
- [x] Ability to mark columns as keys

<!-- Links -->
[keep a changelog]: https://keepachangelog.com/en/1.0.0/
[semantic versioning]: https://semver.org/spec/v2.0.0.html
