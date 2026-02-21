use crate::data::*;
use crate::view::*;
use egui::*;

pub struct Project {
    pub configuration: ProjectConfiguration,
}

impl Project {
    pub fn new(name: String) -> Self {
        Self {
            configuration: ProjectConfiguration {
                name,
                data_sources: vec![],
            },
        }
    }

    pub fn rename(&mut self, new_name: String) {
        self.configuration.name = new_name;
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
                    ui.ctx().data_mut(|d| d.insert_temp(Id::new("trash_project_index"), Some(index)));
                    ui.close();
                }
            }
            Rename::DataSource(index) => {
                if ui.button("Remove").clicked() {
                    ui.ctx().data_mut(|d| d.insert_temp(Id::new("trash_datasource_index"), Some(index)));
                    ui.close();
                }
            }
            _ => {}
        }
    }

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
    ) {
        let renaming_target_id = Id::new("renaming_target");
        let res = ui.selectable_label(selected, format!("{} {}", icon, display_name));
        let res = if let Some(hover) = hover_text {
            res.on_hover_text(hover)
        } else {
            res
        };

        res.context_menu(|ui| {
            Self::ui_item_context_menu(ui, target);
        });

        if res.clicked() {
            on_click();
        }

        if res.double_clicked() {
            ui.data_mut(|d| d.insert_temp(renaming_target_id, target));
        }
    }

    pub fn ui(
        &self,
        ui: &mut Ui,
        project_idx: usize,
        renaming_target_opt: Option<Rename>,
        view_model: &mut RootViewModel,
    ) {
        let renaming_this_project =
            renaming_target_opt.map_or(false, |t| t == Rename::Project(project_idx));

        if renaming_this_project {
            Self::ui_item_as_editable(
                ui,
                view_model,
                Rename::Project(project_idx),
                ui.id().with("rename_project"),
                egui_material_icons::icons::ICON_ASSIGNMENT,
                &self.configuration.name,
            );
        } else {
            Self::ui_item_as_selectable(
                ui,
                Rename::Project(project_idx),
                false,
                egui_material_icons::icons::ICON_ASSIGNMENT,
                &self.configuration.name,
                None,
                || {},
            );
        }
    }
}
