use std::borrow::Cow;
use eframe::epaint::text::TextWrapMode;
use egui::{Key, PopupCloseBehavior};
use egui_data_table::viewer::{HeaderAction, HeaderResult};
use crate::data::*;

pub struct ColumnHeader<'a> {
    pub column_configs: &'a mut Vec<ColumnConfig>,
}

impl<'a> ColumnHeader<'a> {
    pub fn new(column_configs: &'a mut Vec<ColumnConfig>) -> Self {
        Self { column_configs }
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

        let config = &self.column_configs[column];

        let mut action = None;
        ui.horizontal(|ui| {
            ui.label(egui_material_icons::icons::ICON_NOTES);

            let initial_name = config.display_name.as_ref().unwrap_or(&config.name).clone();

            let mut current_name = ui.data_mut(|d| d.get_temp::<String>(ui.id().with("rename")).unwrap_or(initial_name));

            ui.text_edit_singleline(&mut current_name);
            if ui.input(|i| i.key_pressed(Key::Enter)) {
                action = Some(HeaderAction::RenameCommitted(current_name.clone()));
                ui.data_mut(|d| d.remove::<String>(ui.id().with("rename")));
                ui.close();
            } else if ui.input(|i| i.key_pressed(Key::Escape)) {
                ui.data_mut(|d| d.remove::<String>(ui.id().with("rename")));
                ui.close();
            } else {
                ui.data_mut(|d| d.insert_temp(ui.id().with("rename"), current_name));
            }

        });

        ui.separator();

        if config.column_type == ColumnType::Relation {
            
            ui.menu_button("Related source", |ui| {
                let current_source = self.column_configs[column].related_source.clone().unwrap_or_default();

                let mut available_sources = Vec::new();
                for ds in &data_sources {
                    let source_name = ds.name.clone().unwrap_or_else(|| ds.path.clone());
                    for sheet in &ds.sheets {
                        let sheet_name = sheet.display_name.as_ref().unwrap_or(&sheet.name);
                        available_sources.push(format!("{} > {}", source_name, sheet_name));
                    }
                }

                for source in available_sources {
                    let mut is_selected = current_source == source;
                    if ui.checkbox(&mut is_selected, &source).clicked() {
                        if is_selected {
                            self.column_configs[column].related_source = Some(source.clone());
                        } else {
                            self.column_configs[column].related_source = None;
                        }
                        action = Some(HeaderAction::RequestSave);
                    }
                }
            });
        }

        ui.menu_button(format!("{} Change type", egui_material_icons::icons::ICON_EDIT_SQUARE), |ui| {
            let current_type = self.column_configs[column].column_type;

            let mut is_text = current_type == ColumnType::Text;
            if ui.checkbox(&mut is_text, format!("{} Text", ColumnType::Text.icon())).clicked() {
                self.column_configs[column].column_type = ColumnType::Text;
                action = Some(HeaderAction::RequestSave);
                //ui.close();
            }
            let mut is_number = current_type == ColumnType::Number;
            if ui.checkbox(&mut is_number, format!("{} Number", ColumnType::Number.icon())).clicked() {
                self.column_configs[column].column_type = ColumnType::Number;
                action = Some(HeaderAction::RequestSave);
                //ui.close();
            }
            let mut is_date_time = current_type == ColumnType::DateTime;
            if ui.checkbox(&mut is_date_time, format!("{} Date / time", ColumnType::DateTime.icon())).clicked() {
                self.column_configs[column].column_type = ColumnType::DateTime;
                action = Some(HeaderAction::RequestSave);
                //ui.close();
            }

            let is_virtual = self.column_configs[column].is_virtual;
            ui.add_enabled_ui(is_virtual, |ui| {
                let mut is_select = current_type == ColumnType::Select;
                if ui.checkbox(&mut is_select, format!("{} Select", ColumnType::Select.icon())).clicked() {
                    self.column_configs[column].column_type = ColumnType::Select;
                    action = Some(HeaderAction::RequestSave);
                    //ui.close();
                }
                let mut is_multi_select = current_type == ColumnType::MultiSelect;
                if ui.checkbox(&mut is_multi_select, format!("{} Multi-select", ColumnType::MultiSelect.icon())).clicked() {
                    self.column_configs[column].column_type = ColumnType::MultiSelect;
                    action = Some(HeaderAction::RequestSave);
                    //ui.close();
                }
                let mut is_relation = current_type == ColumnType::Relation;
                if ui.checkbox(&mut is_relation, format!("{} Relation", ColumnType::Relation.icon())).clicked() {
                    self.column_configs[column].column_type = ColumnType::Relation;
                    action = Some(HeaderAction::RequestSave);
                    //ui.close();
                }
            });

            ui.add_enabled_ui(false, |ui| {
                if ui.button(format!("{} Status", egui_material_icons::icons::ICON_TARGET)).clicked() {
                    ui.close();
                }
                if ui.button(format!("{} Person", egui_material_icons::icons::ICON_GROUP)).clicked() {
                    ui.close();
                }
                if ui.button(format!("{} URL", egui_material_icons::icons::ICON_LINK)).clicked() {
                    ui.close();
                }
                if ui.button(format!("{} Email", egui_material_icons::icons::ICON_ALTERNATE_EMAIL)).clicked() {
                    ui.close();
                }
                if ui.button(format!("{} Phone", egui_material_icons::icons::ICON_CALL)).clicked() {
                    ui.close();
                }
                // Moved Relation to enabled section above.
                if ui.button(format!("{} Rollup", egui_material_icons::icons::ICON_SEARCH)).clicked() {
                    ui.close();
                }
                if ui.button(format!("{} Created time", egui_material_icons::icons::ICON_NEST_CLOCK_FARSIGHT_ANALOG)).clicked() {
                    ui.close();
                }
                if ui.button(format!("{} Created by", egui_material_icons::icons::ICON_ACCOUNT_CIRCLE)).clicked() {
                    ui.close();
                }
                if ui.button(format!("{} Last edited time", egui_material_icons::icons::ICON_NEST_CLOCK_FARSIGHT_ANALOG)).clicked() {
                    ui.close();
                }
                if ui.button(format!("{} Last edited by", egui_material_icons::icons::ICON_ACCOUNT_CIRCLE)).clicked() {
                    ui.close();
                }
                if ui.button(format!("{} Location", egui_material_icons::icons::ICON_LOCATION_ON)).clicked() {
                    ui.close();
                }
            });
        });

        let is_name_active = self.column_configs[column].is_name;
        let is_key_active = self.column_configs[column].is_key;

        ui.separator(); // ========================================

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
        });

        if ui.button(format!("{} Hide", egui_material_icons::icons::ICON_VISIBILITY_OFF)).clicked() {
            ui.close();
        }

        let mut is_key = is_key_active;
        if ui.checkbox(&mut is_key, "Use as key").clicked() {
            self.column_configs[column].is_key = is_key;
            action = Some(HeaderAction::RequestSave);
            ui.close();
        }

        let mut is_name = is_name_active;
        if ui.checkbox(&mut is_name, "Use as name").clicked() {
            if is_name {
                // Turn off is_name for all other columns
                for c in self.column_configs.iter_mut() {
                    c.is_name = false;
                }
                self.column_configs[column].is_name = true;
            } else {
                self.column_configs[column].is_name = false;
            }
            action = Some(HeaderAction::RequestSave);
            ui.close();
        }

        ui.separator();

        if ui.button(format!("{} Insert left", egui_material_icons::icons::ICON_ADD_COLUMN_LEFT)).clicked() {
            action = Some(HeaderAction::AddColumn(column));
            ui.close();
        }
        if ui.button(format!("{} Insert right", egui_material_icons::icons::ICON_ADD_COLUMN_RIGHT)).clicked() {
            action = Some(HeaderAction::AddColumn(column + 1));
            ui.close();
        }


        ui.separator();

        if column > 0 {
            if ui.button("Move Left").clicked() {
                action = Some(HeaderAction::MoveColumn(column, column - 1));
                ui.close();
            }
        }
        if column < self.column_configs.len() - 1 {
            if ui.button("Move Right").clicked() {
                action = Some(HeaderAction::MoveColumn(column, column + 1));
                ui.close();
            }
        }

        ui.separator();

        if ui.button(format!("{} Duplicate", egui_material_icons::icons::ICON_STACK)).clicked() {
            ui.close();
        }
        if ui.button(format!("{} Trash", egui_material_icons::icons::ICON_DELETE)).clicked() {
            ui.close();
        }

        action
    }
}
