use crate::data::*;

#[derive(Clone)]
pub struct DataSource {
    pub path: String,
    pub name: Option<String>,
    pub config: SourceConfig,
    pub sheets: Vec<DataSheet>,
    pub selected_sheet_index: usize,
}

impl DataSource {
    pub fn new(
        path: String,
        name: Option<String>,
        config: SourceConfig,
        sheets: Vec<DataSheet>,
        selected_sheet_index: usize,
    ) -> Self {
        Self {
            path,
            name,
            config,
            sheets,
            selected_sheet_index,
        }
    }

    pub fn save(
        &mut self,
        column_configs: Vec<ColumnConfig>,
        table: egui_data_table::DataTable<Row>,
    ) -> Result<(), String> {
        self.sheets[self.selected_sheet_index].column_configs = column_configs;
        self.sheets[self.selected_sheet_index].table = table;

        let mut sheet_configs = Vec::new();
        for sheet in &mut self.sheets {
            for (i, config) in sheet.column_configs.iter_mut().enumerate() {
                config.order = i;
            }

            let key_col_idx = sheet.column_configs.iter().position(|c| c.is_key);
            let virtual_cols: Vec<usize> = sheet.column_configs.iter().enumerate()
                .filter(|(_, c)| c.is_virtual)
                .map(|(i, _)| i)
                .collect();

            let mut cell_values = Vec::new();
            if let Some(key_idx) = key_col_idx {
                let rows: &Vec<Row> = &sheet.table;
                for row in rows {
                    let key = row.cells[key_idx].0.clone();
                    if key.is_empty() {
                        continue;
                    }

                    for &v_idx in &virtual_cols {
                        let value = row.cells[v_idx].0.clone();
                        if !value.is_empty() {
                            cell_values.push(CellValueConfig {
                                key: key.clone(),
                                column_name: sheet.column_configs[v_idx].name.clone(),
                                value,
                            });
                        }
                    }
                }
            }

            sheet_configs.push(SheetConfig {
                name: sheet.name.clone(),
                display_name: sheet.display_name.clone(),
                column_configs: sheet.column_configs.clone(),
                sort_config: None,
                cell_values,
            });
        }

        self.config.name = self.name.clone();
        self.config.sheets = sheet_configs;
        self.config.save()
    }
}