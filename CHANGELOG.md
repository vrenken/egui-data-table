# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog],
and this project adheres to [Semantic Versioning].

## [Unreleased]

### Added
- Ability to mark a column as the "name" column from the column header context menu. This column is marked with a 🏷️ icon.

### Fixed
- Fixed an issue where renamed data sources and sheets were not persisted to the `.correlate` file.

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
