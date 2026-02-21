use egui::*;
use crate::view::*;
use crate::data::*;

#[derive(Default)]
pub struct HierarchyPanel {}

impl HierarchyPanel {
    pub fn ui(&mut self, view_model: &mut RootViewModel, ctx: &Context) -> (Option<usize>, Option<usize>) {
        let mut newly_selected_index = None;
        let mut newly_selected_sheet_index = None;

        SidePanel::left("hierarchy_panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.heading("Project");
                    ui.separator();

                    let header_res = CollapsingHeader::new(format!("{} Projects", egui_material_icons::icons::ICON_FOLDER))
                        .default_open(true)
                        .show(ui, |ui| {
                            
                            let renaming_target_id = Id::new("renaming_target");
                            let renaming_target_opt = ui.data(|d| d.get_temp::<Rename>(renaming_target_id));

                            if let Some(project_configs) = view_model.config.projects.clone() {
                                for project_idx in 0..project_configs.len() {
                                    let project = Project {
                                        configuration: project_configs[project_idx].clone(),
                                    };
                                    project.ui(ui, project_idx, renaming_target_opt, view_model, &mut newly_selected_index, &mut newly_selected_sheet_index);
                                }
                            }

                            ui.add_space(10.0);
                            ui.label("Data sources:");

                            let assigned_data_sources: std::collections::HashSet<String> = view_model.config.projects.as_ref()
                                .map(|projects| projects.iter().flat_map(|p| p.data_sources.clone()).collect())
                                .unwrap_or_default();

                            let mut any_general_ds = false;
                            for index in 0..view_model.data_sources.len() {
                                if !assigned_data_sources.contains(&view_model.data_sources[index].path) {
                                    any_general_ds = true;
                                    view_model.data_sources[index].clone().ui(
                                        ui,
                                        index,
                                        renaming_target_opt,
                                        view_model,
                                        &mut newly_selected_index,
                                        &mut newly_selected_sheet_index
                                    );
                                }
                            }

                            if !any_general_ds {
                                ui.label("No unassigned data sources");
                            }
                        });

                    header_res.header_response.context_menu(|ui| {
                        if ui.button("Add project").clicked() {
                            view_model.add_project();
                            ui.close();
                        }
                        ui.separator();
                        if let Some(path) = Self::ui_hierarchy_panel_context_menu(ui) {
                            view_model.pending_file_to_add = Some((path, None));
                        }
                    });

                    ui.add_space(20.);

                    ui.heading("Configuration");
                    ui.separator();

                    if ui.button("💾 Save as default").clicked() {
                        view_model.config.data_sources = view_model.data_sources.iter().map(|ds| ds.path.clone()).collect();
                        view_model.config.selected_index = view_model.selected_index;
                        if let Err(e) = view_model.config.save() {
                            log::error!("Failed to save config: {}", e);
                        }
                        // Also save .correlate files for all data sources
                        for i in 0..view_model.data_sources.len() {
                            view_model.save_source_config(i);
                        }
                    }

                    ui.add_space(20.);

                    ui.heading("Hotkeys");
                    ui.separator();
                    ui.add_space(0.);
                    for (k, a) in &view_model.viewer.hotkeys {
                        Button::new(format!("{a:?}"))
                            .shortcut_text(ctx.format_shortcut(k))
                            .wrap_mode(TextWrapMode::Wrap)
                            .sense(Sense::hover())
                            .ui(ui);
                    }
                });
            });

        (newly_selected_index, newly_selected_sheet_index)
    }

    pub fn ui_hierarchy_panel_context_menu(ui: &mut Ui) -> Option<std::path::PathBuf> {
        let mut result = None;
        if ui.button("Add data source").clicked() {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("Excel Files", &["xlsx"])
                .add_filter("CSV Files", &["csv"])
                .pick_file() 
            {
                result = Some(path);
            }
            ui.close();
        }
        result
    }
}
