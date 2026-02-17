use std::borrow::Cow;
use egui::Key;
use egui_data_table::viewer::{HeaderAction, HeaderResult};
use crate::data::*;

pub struct ColumnHeader<'a> {
    pub column_configs: &'a mut Vec<ColumnConfig>,
    pub visible_columns: Option<Vec<usize>>, // indices of visible columns in order
}

impl<'a> ColumnHeader<'a> {
    pub fn new(column_configs: &'a mut Vec<ColumnConfig>) -> Self {
        Self { column_configs, visible_columns: None }
    }

    pub fn new_with_visibility(column_configs: &'a mut Vec<ColumnConfig>, visible_columns: Option<Vec<usize>>) -> Self {
        Self { column_configs, visible_columns }
    }

    pub fn name(&self, column: usize) -> Cow<'static, str> {
        self.column_configs.get(column)
            .map(|c| {
                let mut name = c.display_name.as_ref().unwrap_or(&c.name).clone();
                if c.is_key {
                    name = format!("{} {}", name, egui_material_icons::icons::ICON_KEY);
                }
                if c.is_name {
                    name = format!("{} {}", name, egui_material_icons::icons::ICON_VISIBILITY);
                }
                if c.is_virtual {
                    name = format!("{} {}", name, egui_material_icons::icons::ICON_SYRINGE);
                }

                let type_icon = c.column_type.icon();
                name = format!("{} {}", type_icon, name);

                Cow::Owned(name)
            })
            .unwrap_or_else(|| Cow::Owned(format!("Column {}", column)))
    }

    pub fn show(&self, ui: &mut egui::Ui, column: usize) {
        ui.add(egui::Label::new(self.name(column)).selectable(false));
    }

    pub fn context_menu(&mut self, ui: &mut egui::Ui, column: usize, data_sources: Vec<DataSource>) -> HeaderResult {
        let mut action = None;

        self.show_rename_section(ui, column, &mut action);
        ui.separator();

        self.show_relation_section(ui, column, &data_sources, &mut action);
        self.show_change_type_section(ui, column, &mut action);

        ui.separator();

        self.show_filter_sort_hide_section(ui, column, &mut action);
        self.show_key_name_toggles(ui, column, &mut action);

        ui.separator();

        self.show_insert_section(ui, column, &mut action);
        ui.separator();

        self.show_move_section(ui, column, &mut action);
        ui.separator();

        self.show_footer_section(ui, column);

        action
    }

    fn show_rename_section(&mut self, ui: &mut egui::Ui, column: usize, action: &mut HeaderResult) {
        let config = &self.column_configs[column];
        ui.horizontal(|ui| {
            ui.label(egui_material_icons::icons::ICON_NOTES);

            let initial_name = config.display_name.as_ref().unwrap_or(&config.name).clone();
            let mut current_name = ui.data_mut(|d| d.get_temp::<String>(ui.id().with("rename")).unwrap_or(initial_name));

            ui.text_edit_singleline(&mut current_name);
            if ui.input(|i| i.key_pressed(Key::Enter)) {
                *action = Some(HeaderAction::RenameCommitted(current_name.clone()));
                ui.data_mut(|d| d.remove::<String>(ui.id().with("rename")));
                ui.close();
            } else if ui.input(|i| i.key_pressed(Key::Escape)) {
                ui.data_mut(|d| d.remove::<String>(ui.id().with("rename")));
                ui.close();
            } else {
                ui.data_mut(|d| d.insert_temp(ui.id().with("rename"), current_name));
            }
        });
    }

    fn show_relation_section(&mut self, ui: &mut egui::Ui, column: usize, data_sources: &[DataSource], action: &mut HeaderResult) {
        let config = &self.column_configs[column];
        if config.column_type == ColumnType::Relation {
            ui.menu_button("Related source", |ui| {
                let current_source = self.column_configs[column].related_source.clone().unwrap_or_default();

                let mut available_sources = Vec::new();
                for ds in data_sources {
                    let source_name = ds.name.as_ref().unwrap_or(&ds.path);
                    for sheet in &ds.sheets {
                        let sheet_name = sheet.display_name.as_ref().unwrap_or(&sheet.name);
                        available_sources.push(format!("{} > {}", source_name, sheet_name));
                    }
                }

                for source in available_sources {
                    let mut is_selected = current_source == source;
                    if ui.checkbox(&mut is_selected, &source).clicked() {
                        self.column_configs[column].related_source = if is_selected { Some(source) } else { None };
                        *action = Some(HeaderAction::RequestSave);
                    }
                }
            });
        }
    }

    fn show_change_type_section(&mut self, ui: &mut egui::Ui, column: usize, action: &mut HeaderResult) {
        ui.menu_button(format!("{} Change type", egui_material_icons::icons::ICON_EDIT_SQUARE), |ui| {
            let current_type = self.column_configs[column].column_type;

            let types = [
                ColumnType::Text,
                ColumnType::Number,
                ColumnType::DateTime,
            ];

            for t in types {
                let mut is_selected = current_type == t;
                if ui.checkbox(&mut is_selected, format!("{} {:?}", t.icon(), t)).clicked() {
                    self.column_configs[column].column_type = t;
                    *action = Some(HeaderAction::RequestSave);
                }
            }

            let is_virtual = self.column_configs[column].is_virtual;
            ui.add_enabled_ui(is_virtual, |ui| {
                let virtual_types = [
                    ColumnType::Select,
                    ColumnType::MultiSelect,
                    ColumnType::Relation,
                ];
                for t in virtual_types {
                    let mut is_selected = current_type == t;
                    if ui.checkbox(&mut is_selected, format!("{} {:?}", t.icon(), t)).clicked() {
                        self.column_configs[column].column_type = t;
                        *action = Some(HeaderAction::RequestSave);
                    }
                }
            });

            self.show_disabled_types(ui);
        });
    }

    fn show_disabled_types(&self, ui: &mut egui::Ui) {
        ui.add_enabled_ui(false, |ui| {
            let disabled = [
                (egui_material_icons::icons::ICON_TARGET, "Status"),
                (egui_material_icons::icons::ICON_GROUP, "Person"),
                (egui_material_icons::icons::ICON_LINK, "URL"),
                (egui_material_icons::icons::ICON_ALTERNATE_EMAIL, "Email"),
                (egui_material_icons::icons::ICON_CALL, "Phone"),
                (egui_material_icons::icons::ICON_SEARCH, "Rollup"),
                (egui_material_icons::icons::ICON_NEST_CLOCK_FARSIGHT_ANALOG, "Created time"),
                (egui_material_icons::icons::ICON_ACCOUNT_CIRCLE, "Created by"),
                (egui_material_icons::icons::ICON_NEST_CLOCK_FARSIGHT_ANALOG, "Last edited time"),
                (egui_material_icons::icons::ICON_ACCOUNT_CIRCLE, "Last edited by"),
                (egui_material_icons::icons::ICON_LOCATION_ON, "Location"),
            ];
            for (icon, label) in disabled {
                if ui.button(format!("{} {}", icon, label)).clicked() {
                    ui.close();
                }
            }
        });
    }

    fn show_filter_sort_hide_section(&mut self, ui: &mut egui::Ui, column: usize, action: &mut HeaderResult) {
        if ui.button(format!("{} Filter", egui_material_icons::icons::ICON_FILTER_LIST)).clicked() {
            ui.close();
        }

        ui.menu_button(format!("{} Sort", egui_material_icons::icons::ICON_SWAP_VERT), |ui| {
            if ui.button(format!("{} Sort ascending", egui_material_icons::icons::ICON_NORTH)).clicked() {
                ui.close();
            }
            if ui.button(format!("{} Sort descending", egui_material_icons::icons::ICON_SOUTH)).clicked() {
                ui.close();
            }
            if ui.button("Clear sort").clicked() {
                *action = Some(HeaderAction::ClearSort);
                ui.close();
            }
        });

        if ui.button(format!("{} Hide", egui_material_icons::icons::ICON_VISIBILITY_OFF)).clicked() {
            *action = Some(HeaderAction::HideColumn(column));
            ui.close();
        }

        // Hidden columns submenu
        if let Some(vis) = &self.visible_columns {
            let total = self.column_configs.len();
            let visible_set: std::collections::HashSet<usize> = vis.iter().cloned().collect();
            let hidden: Vec<usize> = (0..total).filter(|i| !visible_set.contains(i)).collect();
            if !hidden.is_empty() {
                ui.separator();
                ui.menu_button("Show", |ui| {
                    for idx in hidden {
                        if ui.button(self.name(idx)).clicked() {
                            *action = Some(HeaderAction::ShowHidden(idx));
                            ui.close();
                        }
                    }
                });
            }
        }
    }

    fn show_key_name_toggles(&mut self, ui: &mut egui::Ui, column: usize, action: &mut HeaderResult) {
        let is_name_active = self.column_configs[column].is_name;
        let is_key_active = self.column_configs[column].is_key;

        let mut is_key = is_key_active;
        if ui.checkbox(&mut is_key, "Use as key").clicked() {
            self.column_configs[column].is_key = is_key;
            *action = Some(HeaderAction::RequestSave);
            ui.close();
        }

        let mut is_name = is_name_active;
        if ui.checkbox(&mut is_name, "Use as name").clicked() {
            for c in self.column_configs.iter_mut() {
                c.is_name = false;
            }
            self.column_configs[column].is_name = is_name;
            *action = Some(HeaderAction::RequestSave);
            ui.close();
        }
    }

    fn show_insert_section(&mut self, ui: &mut egui::Ui, column: usize, action: &mut HeaderResult) {
        if ui.button(format!("{} Insert left", egui_material_icons::icons::ICON_ADD_COLUMN_LEFT)).clicked() {
            *action = Some(HeaderAction::AddColumn(column));
            ui.close();
        }
        if ui.button(format!("{} Insert right", egui_material_icons::icons::ICON_ADD_COLUMN_RIGHT)).clicked() {
            *action = Some(HeaderAction::AddColumn(column + 1));
            ui.close();
        }
    }

    fn show_move_section(&mut self, ui: &mut egui::Ui, column: usize, action: &mut HeaderResult) {
        if column > 0 {
            if ui.button("Move Left").clicked() {
                *action = Some(HeaderAction::MoveColumn(column, column - 1));
                ui.close();
            }
        }
        if column < self.column_configs.len() - 1 {
            if ui.button("Move Right").clicked() {
                *action = Some(HeaderAction::MoveColumn(column, column + 1));
                ui.close();
            }
        }
    }

    fn show_footer_section(&mut self, ui: &mut egui::Ui, _column: usize) {
        if ui.button(format!("{} Duplicate", egui_material_icons::icons::ICON_STACK)).clicked() {
            ui.close();
        }
        if ui.button(format!("{} Trash", egui_material_icons::icons::ICON_DELETE)).clicked() {
            ui.close();
        }
    }
}
