use crate::data::*;
use egui_data_table::DataTable;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Rename {
    Project(usize),
    DataSource(usize),
    Sheet(usize, usize),
    Row(usize),
    Column(usize),
}

impl Rename
{
    pub fn apply(
        target: Rename,
        new_name: String,
        config: &mut Configuration,
        data_sources: &mut [DataSource],
        table: &mut DataTable<Row>,
        column_configs: &mut [ColumnConfiguration],
    ) {
        match target {
            Rename::Project(project_idx) => {
                if let Some(project_configs) = config.projects.as_mut() {
                    if let Some(project_config) = project_configs.get_mut(project_idx) {
                        let mut project = Project {
                            configuration: project_config.clone(),
                        };
                        project.rename(new_name);
                        *project_config = project.configuration;
                        if let Err(e) = config.save() {
                            log::error!("Failed to save config after project rename: {}", e);
                        }
                    }
                }
            }
            Rename::DataSource(ds_idx) => {
                if let Some(ds) = data_sources.get_mut(ds_idx) {
                    ds.name = if new_name.is_empty() {
                        None
                    } else {
                        Some(new_name)
                    };
                }
            }
            Rename::Sheet(ds_idx, sheet_idx) => {
                if let Some(ds) = data_sources.get_mut(ds_idx) {
                    if let Some(sheet) = ds.sheets.get_mut(sheet_idx) {
                        sheet.display_name = if new_name.is_empty() || new_name == sheet.name {
                            None
                        } else {
                            Some(new_name)
                        };
                    }
                }
            }
            Rename::Row(row_idx) => {
                let name_col_idx = column_configs
                    .iter()
                    .position(|c| c.is_name)
                    .or_else(|| column_configs.iter().position(|c| c.name.contains("Name")))
                    .or_else(|| {
                        column_configs
                            .iter()
                            .position(|c| c.column_type == ColumnType::Text)
                    })
                    .unwrap_or(0);

                if let Some(row) = table.get_mut(row_idx) {
                    row.cells[name_col_idx] = CellValue::from(new_name);
                }
            }
            Rename::Column(col_idx) => {
                if let Some(config) = column_configs.get_mut(col_idx) {
                    config.display_name = if new_name.is_empty() || new_name == config.name {
                        None
                    } else {
                        Some(new_name)
                    };
                    if config.is_virtual {
                        config.name = config
                            .display_name
                            .clone()
                            .unwrap_or_else(|| config.name.clone());
                    }
                }
            }
        }
    }
}

