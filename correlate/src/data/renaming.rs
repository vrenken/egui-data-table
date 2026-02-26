use crate::data::*;
use crate::view::*;
use egui::*;
use crate::egui_data_table::*;
use crate::enqueue_ui_command;

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
    pub fn ui_item_as_editable(
        ui: &mut Ui,
        view_model: &mut RootViewModel,
        target: Rename,
        rename_id: Id,
        icon: &str,
        current_name: &str,
    ) {
        let renaming_target_id = Id::new("renaming_target");
        ui.horizontal(|ui| {
            ui.label(format!("{} ", icon));
            let mut name = ui.data_mut(|d| d.get_temp::<String>(rename_id).unwrap_or_else(|| current_name.to_string()));
            let res = ui.text_edit_singleline(&mut name);

            res.context_menu(|ui| {
                Self::ui_item_context_menu(ui, target);
            });

            if res.lost_focus() || (ui.input(|i| i.key_pressed(Key::Enter))) {
                view_model.apply_rename(target, name.clone());
                ui.data_mut(|d| {
                    d.remove::<Rename>(renaming_target_id);
                    d.remove::<String>(rename_id);
                });
            } else if ui.input(|i| i.key_pressed(Key::Escape)) {
                ui.data_mut(|d| {
                    d.remove::<Rename>(renaming_target_id);
                    d.remove::<String>(rename_id);
                });
            } else {
                ui.data_mut(|d| d.insert_temp(rename_id, name));
            }
            res.request_focus();
        });
    }

    pub fn ui_item_as_selectable(
        ui: &mut Ui,
        target: Rename,
        selected: bool,
        icon: &str,
        display_name: &str,
        hover_text: Option<&str>,
        on_click: impl FnOnce(),
    ) -> Response {
        let renaming_target_id = Id::new("renaming_target");
        let res = ui.selectable_label(selected, format!("{} {}", icon, display_name));
        let res = if let Some(hover) = hover_text {
            res.on_hover_text(hover)
        } else {
            res
        };

        if res.clicked() {
            on_click();
        }

        if res.double_clicked() {
            ui.data_mut(|d| d.insert_temp(renaming_target_id, target));
        }
        res
    }

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
                        project.rename(new_name, config);
                    }
                }
            }
            Rename::DataSource(ds_idx) => {
                if let Some(ds) = data_sources.get_mut(ds_idx) {
                    ds.rename(new_name, config);
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

    pub fn ui_item_context_menu(ui: &mut Ui, target: Rename) {
        let renaming_target_id = Id::new("renaming_target");
        if ui.button("Rename").clicked() {
            ui.data_mut(|d| d.insert_temp(renaming_target_id, target));
            ui.close();
        }
        match target {
            Rename::Project(index) => {
                if ui.button("Remove").clicked() {

                    enqueue_ui_command(ui, Box::new(TrashProject { project: index, ctx: ui.ctx().clone() }));
                    ui.close();
                }
            }
            Rename::DataSource(index) => {
                if ui.button("Remove").clicked() {
                    enqueue_ui_command(ui, Box::new(TrashDataSource { data_source: index, ctx: ui.ctx().clone() }));
                    ui.close();
                }
            }
            _ => {}
        }
    }
}


