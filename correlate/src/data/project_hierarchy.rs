use egui::*;
use crate::data::*;
use crate::view::*;

impl Project {
    pub fn ui(
        &self,
        ui: &mut Ui,
        project_idx: usize,
        renaming_target_opt: Option<Rename>,
        view_model: &mut RootViewModel,
    ) {
        let renaming_this_project = renaming_target_opt.map_or(false, |t| t == Rename::Project(project_idx));

        if renaming_this_project {
            Rename::ui_item_as_editable(
                ui,
                view_model,
                Rename::Project(project_idx),
                ui.id().with("rename_project"),
                egui_material_icons::icons::ICON_ASSIGNMENT,
                &self.configuration.name,
            );
        } else {
            Rename::ui_item_as_selectable(
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