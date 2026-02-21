use crate::data::*;

#[derive(Clone)]
pub struct DataSource {
    pub path: String,
    pub name: Option<String>,
    pub sheets: Vec<DataSheet>,
    pub selected_sheet_index: usize,
}

impl DataSource {
    pub fn save(&mut self) -> Result<(), String> {
        let companion_path = crate::data::SourceConfig::get_companion_path(&self.path);

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

        let source_config = SourceConfig {
            name: self.name.clone(),
            sheets: sheet_configs,
        };
        source_config.save(companion_path)
    }
}