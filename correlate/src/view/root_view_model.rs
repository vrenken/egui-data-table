use crate::data::*;
use crate::view::*;
use crate::egui_data_table::*;

pub struct RootViewModel {
    pub config: Configuration,
    pub table: DataTable<Row>,
    pub viewer: RowView,
    pub data_sources: Vec<DataSource>,
    pub selected_index: Option<usize>,
    pub style_override: Style,
    pub scroll_bar_always_visible: bool,
}

impl RootViewModel {
    pub fn save_source_config(&mut self, index: usize) {
        if let Some(ds) = self.data_sources.get_mut(index) {
            let (configs, table) = if Some(index) == self.selected_index {
                (self.viewer.column_configs.clone(), self.table.clone())
            } else {
                let sheet = &ds.sheets[ds.selected_sheet_index];
                (sheet.column_configs.clone(), sheet.table.clone())
            };

            if let Err(e) = ds.save(configs, table) {
                log::error!("Failed to save companion config for {}: {}", ds.path, e);
            }
        }
    }

    pub fn default(config: Configuration) -> Self {

        let mut data_sources = Vec::new();
        for project in config.projects.as_ref().unwrap_or(&Vec::new()) {
            for source in &project.data_sources {
                let extension = std::path::Path::new(source)
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("");


                let loader: Box<dyn SheetLoader> = if extension == "csv" {
                    Box::new(CsvSheet)
                } else {
                    Box::new(ExcelSheet)
                };

                let loaded = loader.load(source.clone());

                match loaded {
                    Ok((loaded_sheets, source_config)) => {
                        let custom_name = loaded_sheets.first().and_then(|s| s.custom_name.clone());
                        data_sources.push(DataSource::new(
                            source.to_string(),
                            custom_name,
                            source_config,
                            loaded_sheets,
                            0,
                        ));
                    }
                    Err(e) => {
                        log::error!("Failed to load {}: {}", source, e);
                    }
                }
            }
        }

        if data_sources.is_empty() {
            let selected_index = None;
            let table = DataTable::new();
            let viewer = RowView {
                name_filter: String::new(),
                row_protection: false,
                hotkeys: Vec::new(),
                column_configs: Vec::new(),
                config: config.clone(),
                data_sources: data_sources.clone(),
                visible_columns: None,
            };

            return Self {
                config,
                table,
                viewer,
                data_sources,
                selected_index,
                style_override: Default::default(),
                scroll_bar_always_visible: false,
            };
        }

        let selected_index = config.selected_index.unwrap_or(0).min(data_sources.len() - 1);
        let ds = &data_sources[selected_index];
        let sheet = &ds.sheets[ds.selected_sheet_index];
        let table = sheet.table.clone();
        let viewer = RowView {
            name_filter: String::new(),
            row_protection: false,
            hotkeys: Vec::new(),
            column_configs: sheet.column_configs.clone(),
            config: config.clone(),
            data_sources: data_sources.clone(),
            visible_columns: None,
        };

        Self {
            config,
            table,
            viewer,
            data_sources,
            selected_index: Some(selected_index),
            style_override: Default::default(),
            scroll_bar_always_visible: false,
        }
    }

    pub fn handle_pending_file_add(&mut self, path: std::path::PathBuf, index: usize) {
        let path_str = path.to_string_lossy().to_string();

        // If the file doesn't exist, create an empty one (with headers)
        if !path.exists() {
            if let Err(e) = std::fs::write(&path, "Name\n") {
                log::error!("Failed to create new file {}: {}", path_str, e);
                return;
            }
        }

        let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");

        let loaded_result = if extension == "csv" {
            CsvSheet.load(path_str.clone())
        } else {
            ExcelSheet.load(path_str.clone())
        };

        match loaded_result {
            Ok((sheets, source_config)) => {
                let custom_name = sheets.first().and_then(|s| s.custom_name.clone());
                let new_index = self.data_sources.len();

                self.data_sources.push(DataSource::new(
                    path_str.clone(),
                    custom_name,
                    source_config,
                    sheets,
                    0,
                ));

                self.switch_to_source(new_index, 0);

                // Persist to config
                if let Some(projects) = self.config.projects.as_mut() {
                    if let Some(project) = projects.get_mut(index) {
                        project.data_sources.push(path_str);
                    }
                }

                self.config.selected_index = self.selected_index;
                if let Err(e) = self.config.save() {
                    log::error!("Failed to save config: {}", e);
                }
            }
            Err(e) => {
                log::error!("Failed to load {}: {}", path_str, e);
            }
        }
    }

    pub fn add_project(&mut self) {
        let projects = self.config.projects.get_or_insert_with(Vec::new);
        let project = Project::new(format!("New Project {}", projects.len() + 1));
        projects.push(project.configuration);
        if let Err(e) = self.config.save() {
            log::error!("Failed to save config after adding project: {}", e);
        }
    }

    pub fn remove_data_source(&mut self, index: usize) {
        if index < self.data_sources.len() {
            let path_to_remove = self.data_sources[index].path.clone();
            self.data_sources.remove(index);

            // Update the selected index if necessary
            if let Some(selected) = self.selected_index {
                if selected == index {
                    // If we removed the selected one, pick a new one or set to None
                    if self.data_sources.is_empty() {
                        self.selected_index = None;
                        self.table = DataTable::new();
                        self.viewer.column_configs = Vec::new();
                        self.viewer.data_sources = Vec::new();
                    } else {
                        let new_idx = index.min(self.data_sources.len() - 1);
                        self.switch_to_source(new_idx, self.data_sources[new_idx].selected_sheet_index);
                    }
                } else if selected > index {
                    self.selected_index = Some(selected - 1);
                }
            }
        
            // Also remove from any project that might contain it
            if let Some(projects) = self.config.projects.as_mut() {
                for project in projects {
                    project.data_sources.retain(|p| p != &path_to_remove);
                }
            }

            if let Err(e) = self.config.save() {
                log::error!("Failed to save config after removing data source: {}", e);
            }
        }
    }

    pub fn switch_to_source(&mut self, index: usize, sheet_idx: usize) {
        // Save the current table state back to its source
        if let Some(old_idx) = self.selected_index {
            let old_ds = &mut self.data_sources[old_idx];
            let old_sheet = &mut old_ds.sheets[old_ds.selected_sheet_index];
            old_sheet.table = self.table.clone();
            old_sheet.column_configs = self.viewer.column_configs.clone();

            self.save_source_config(old_idx);
        }

        // Switch to the new source
        self.selected_index = Some(index);
        let ds = &mut self.data_sources[index];
        ds.selected_sheet_index = sheet_idx;
        let sheet = &ds.sheets[sheet_idx];
        self.table = sheet.table.clone();
        self.viewer.config = self.config.clone();
        self.viewer.column_configs = sheet.column_configs.clone();
        self.viewer.data_sources = self.data_sources.clone();
    }

    pub fn save_datasource_configuration(&mut self) {
        if let Some(idx) = self.selected_index {
            self.save_source_config(idx);
        }
    }

    pub fn apply_rename(&mut self, target: Rename, new_name: String) {
        Rename::apply(
            target,
            new_name,
            &mut self.config,
            &mut self.data_sources,
            &mut self.table,
            &mut self.viewer.column_configs,
        );
        self.save_datasource_configuration();
    }
}
