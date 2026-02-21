use crate::data::*;

pub trait SheetLoader {
    fn load(&self, path: String) -> Result<(Vec<DataSheet>, SourceConfig), String>;
}

#[derive(Clone)]
pub struct DataSheet {
    pub name: String,
    pub custom_name: Option<String>,
    pub display_name: Option<String>,
    pub icon: &'static str,
    pub column_configs: Vec<ColumnConfig>,
    pub table: egui_data_table::DataTable<Row>,
}
impl DataSheet {
    pub fn new_from_raw_data(
        name: String,
        custom_name: Option<String>,
        icon: &'static str,
        raw_headers: &[String],
        raw_rows: &[Vec<String>],
        config_sheet: Option<&SheetConfig>,
    ) -> (Self, SheetConfig) {
        let mut column_configs = config_sheet
            .map(|s| s.column_configs.clone())
            .unwrap_or_default();

        let display_name = config_sheet.and_then(|s| s.display_name.clone());

        // If not loaded from config, infer them
        if column_configs.is_empty() {
            for (i, header) in raw_headers.iter().enumerate() {
                let sample_value = raw_rows.first().and_then(|r| r.get(i)).map(|s| s.as_str()).unwrap_or("");
                let column_type = ColumnType::infer(header, sample_value);
                column_configs.push(ColumnConfig {
                    name: header.to_string(),
                    display_name: None,
                    column_type,
                    is_key: false,
                    is_name: false,
                    is_virtual: false,
                    is_visible: true,
                    order: i,
                    width: None,
                    allowed_values: None,
                    related_source: None,
                });
            }
        } else {
            column_configs.sort_by_key(|c| c.order);
        }

        let mut rows = Vec::new();
        let cell_values = config_sheet.map(|s| &s.cell_values);

        for row_data in raw_rows {
            // 1. First pass: get the physical key value if it exists
            let row_key = DataSheet::get_row_key(&column_configs, row_data);

            // 2. Second pass: build the row
            let mut cells = Vec::new();
            let mut phys_idx = 0;
            for config in &column_configs {
                let physical_value = if !config.is_virtual {
                    let v = row_data.get(phys_idx).map(|s| s.as_str());
                    phys_idx += 1;
                    v
                } else {
                    None
                };

                cells.push(config.column_type.load(
                    physical_value,
                    config,
                    row_key.as_deref(),
                    cell_values.map(|v| v.as_slice())
                ));
            }
            rows.push(Row { cells });
        }

        let sheet_config = SheetConfig {
            name: name.clone(),
            display_name: display_name.clone(),
            column_configs: column_configs.clone(),
            sort_config: None,
            cell_values: Vec::new(),
        };

        (
            Self {
                name,
                custom_name,
                display_name,
                icon,
                column_configs,
                table: rows.into_iter().collect(),
            },
            sheet_config,
        )
    }

    fn get_row_key(column_configs: &Vec<ColumnConfig>, row_data: &Vec<String>) -> Option<String> {
        let mut row_key = None;
        let mut phys_idx = 0;
        for config in column_configs {
            if !config.is_virtual {
                if config.is_key {
                    row_key = row_data.get(phys_idx).cloned();
                }
                phys_idx += 1;
            }
        }
        row_key
    }
}
