use crate::data::*;
use crate::egui_data_table::*;

pub trait SheetLoader {
    fn load(&self, path: String) -> Result<(Vec<DataSheet>, DataSourceConfiguration), String>;
}

#[derive(Clone)]
pub struct DataSheet {
    pub name: String,
    #[allow(dead_code)] // TODO: Validate
    pub configuration: DataSheetConfiguration,
    pub custom_name: Option<String>,
    pub display_name: Option<String>,
    pub icon: &'static str,
    pub column_configs: Vec<ColumnConfiguration>,
    pub table: DataTable<Row>,
}
impl DataSheet {
    pub fn new_from_raw_data(
        name: String,
        custom_name: Option<String>,
        icon: &'static str,
        raw_headers: &[String],
        raw_rows: &[Vec<String>],
        config_sheet: &DataSheetConfiguration,
    ) -> Self {
        let mut column_configs = config_sheet.column_configs.clone();

        let display_name = config_sheet.display_name.clone();

        // If not loaded from config, infer them
        if column_configs.is_empty() {
            for (i, header) in raw_headers.iter().enumerate() {
                let sample_value = raw_rows.first().and_then(|r| r.get(i)).map(|s| s.as_str()).unwrap_or("");
                let column_type = ColumnType::infer(header, sample_value);
                column_configs.push(ColumnConfiguration {
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
        let cell_values = config_sheet.cell_values.clone();

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

                let column_type = config.column_type.load(
                    physical_value,
                    config,
                    row_key.as_deref(),
                    cell_values.clone(),
                );
                
                cells.push(column_type);
            }
            rows.push(Row { cells });
        }

        let configuration = DataSheetConfiguration {
            name: name.clone(),
            display_name: display_name.clone(),
            column_configs: column_configs.clone(),
            sort_config: None,
            cell_values: Vec::new(),
        };

        Self {
            name,
            configuration: configuration.clone(),
            custom_name,
            display_name,
            icon,
            column_configs,
            table: rows.into_iter().collect(),
        }
    }

    fn get_row_key(column_configs: &Vec<ColumnConfiguration>, row_data: &Vec<String>) -> Option<String> {
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
