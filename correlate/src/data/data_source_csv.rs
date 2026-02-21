use csv::ReaderBuilder;
use crate::data::*;

pub struct CsvSheet;

impl Default for CsvSheet {
    fn default() -> Self {
        Self {
        }
    }
}

impl SheetLoader for CsvSheet {
    fn load(&self, path: String) -> Result<(Vec<DataSheet>, DataSourceConfiguration), String> {
        let file_name = std::path::Path::new(&path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("CSV Data")
            .to_string();

        let mut source_config = DataSourceConfiguration::load(&path);
        let custom_name = source_config.name.clone();

        let mut data_sheets = Vec::new();
        let mut sheet_configs = Vec::new();

        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .from_path(&path)
            .map_err(|e| e.to_string())?;

        let headers: Vec<String> = reader.headers()
            .map_err(|e| e.to_string())?
            .iter()
            .map(|s| s.to_string())
            .collect();

        let mut raw_rows = Vec::new();
        for result in reader.records() {
            let record = result.map_err(|e| e.to_string())?;
            let row: Vec<String> = record.iter().map(|s| s.to_string()).collect();
            raw_rows.push(row);
        }

        let sheet_config = source_config.sheets
            .iter()
            .find(|s| s.name == file_name)
            .unwrap();

        let data_sheet = DataSheet::new_from_raw_data(
            file_name,
            custom_name,
            egui_material_icons::icons::ICON_CSV,
            &headers,
            &raw_rows,
            &sheet_config,
        );

        data_sheets.push(data_sheet);
        sheet_configs.push(sheet_config.clone());

        source_config.sheets = sheet_configs;
        if let Err(e) = source_config.save() {
            log::error!("Failed to save config for {}: {}", path, e);
        }
        Ok((data_sheets, source_config))
    }
}
