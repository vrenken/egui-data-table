use umya_spreadsheet::*;
use crate::data::*;

pub struct ExcelSheet;

impl Default for ExcelSheet {
    fn default() -> Self {
        Self {
        }
    }   
}

impl SheetLoader for ExcelSheet {
    fn load(&self, path: String) -> Result<(Vec<DataSheet>, DataSourceConfiguration), String> {
        let book = reader::xlsx::read(&path).map_err(|e| e.to_string())?;

        let mut source_config = DataSourceConfiguration::load(&path);
        let custom_name = source_config.name.clone();

        let mut data_sheets = Vec::new();
        let mut sheet_configs = Vec::new();

        for sheet_idx in 0..book.get_sheet_count() {
            let sheet = book.get_sheet(&sheet_idx).ok_or(format!("Sheet {} not found", sheet_idx))?;
            let sheet_name = sheet.get_name().to_string();

            let (max_col, max_row) = sheet.get_highest_column_and_row();

            let mut headers = Vec::new();
            for col_idx in 1..=max_col {
                headers.push(sheet.get_formatted_value((col_idx, 1)));
            }

            let mut raw_rows = Vec::new();
            for row_idx in 2..=max_row {
                let mut row = Vec::new();
                for col_idx in 1..=max_col {
                    row.push(sheet.get_formatted_value((col_idx, row_idx)));
                }
                raw_rows.push(row);
            }

            let config_sheet = source_config.sheets.iter().find(|s| s.name == sheet_name);

            let (data_sheet, sheet_config) = DataSheet::new_from_raw_data(
                sheet_name,
                custom_name.clone(),
                egui_material_icons::icons::ICON_TABLE_CHART,
                &headers,
                &raw_rows,
                config_sheet,
            );

            data_sheets.push(data_sheet);
            sheet_configs.push(sheet_config);
        }

        source_config.sheets = sheet_configs;
        if let Err(e) = source_config.save() {
            log::error!("Failed to save config for {}: {}", path, e);
        }
        Ok((data_sheets, source_config))
    }
}
