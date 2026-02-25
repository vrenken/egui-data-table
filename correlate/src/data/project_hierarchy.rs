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
        newly_selected_index: &mut Option<usize>,
        newly_selected_sheet_index: &mut Option<usize>,
    ) {
        let renaming_this_project = renaming_target_opt.map_or(false, |t| t == Rename::Project(project_idx));

        let mut header = CollapsingHeader::new(format!("{} {}", egui_material_icons::icons::ICON_ASSIGNMENT, self.configuration.name))
            .default_open(true);

        if renaming_this_project {
             header = CollapsingHeader::new(format!("{} ", egui_material_icons::icons::ICON_ASSIGNMENT));
        }

        let header_res = header.show(ui, |ui| {
            for ds_path in &self.configuration.data_sources {
                if let Some(ds_idx) = view_model.data_sources.iter().position(|ds| &ds.path == ds_path) {
                    view_model.data_sources[ds_idx].clone().ui(
                        ui,
                        ds_idx,
                        renaming_target_opt,
                        view_model,
                        newly_selected_index,
                        newly_selected_sheet_index,
                    );
                }
            }
        });

        if renaming_this_project {
            let mut rect = header_res.header_response.rect;
            rect.min.x += 20.0; // Offset for icon
            Self::show_context_menu(header_res, project_idx, view_model);

            ui.scope_builder(UiBuilder::new().max_rect(rect), |ui| {
                Rename::ui_item_as_editable(
                    ui,
                    view_model,
                    Rename::Project(project_idx),
                    ui.id().with("rename_project"),
                    "",
                    &self.configuration.name,
                );
            });
        } else {
            Self::show_context_menu(header_res, project_idx, view_model);
        }
    }

    pub fn show_context_menu(header_res: CollapsingResponse<()>, project_idx: usize, view_model: &mut RootViewModel)
    {
        header_res.header_response.context_menu(|ui| {
            Rename::ui_item_context_menu(ui, Rename::Project(project_idx));
            ui.separator();
            if let Some(path) = HierarchyPanel::ui_hierarchy_panel_context_menu(ui) {
                view_model.handle_pending_file_add(path, project_idx);
            }
        });
    }
}