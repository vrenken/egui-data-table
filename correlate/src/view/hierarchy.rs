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
                        });

                    header_res.header_response.context_menu(|ui| {
                        if ui.button("Add project").clicked() {
                            view_model.add_project();
                            ui.close();
                        }
                        ui.separator();
                        if let Some(path) = Self::ui_hierarchy_panel_context_menu(ui) {
                            view_model.handle_pending_file_add(path, 0);
                        }
                    });

                    // ui.add_space(20.);
                    //
                    // ui.heading("Configuration");
                    // ui.separator();
                    //
                    // if ui.button("💾 Save as default").clicked() {
                    //     view_model.config.selected_index = view_model.selected_index;
                    //     if let Err(e) = view_model.config.save() {
                    //         log::error!("Failed to save config: {}", e);
                    //     }
                    //     // Also save .correlate files for all data sources
                    //     for i in 0..view_model.data_sources.len() {
                    //         view_model.save_source_config(i);
                    //     }
                    // }



                    // ui.add_space(20.);
                    //
                    // ui.heading("Hotkeys");
                    // ui.separator();
                    // ui.add_space(0.);
                    // for (k, a) in &view_model.viewer.hotkeys {
                    //     Button::new(format!("{a:?}"))
                    //         .shortcut_text(ctx.format_shortcut(k))
                    //         .wrap_mode(TextWrapMode::Wrap)
                    //         .sense(Sense::hover())
                    //         .ui(ui);
                    // }
                });
            });

        (newly_selected_index, newly_selected_sheet_index)
    }

    pub fn ui_hierarchy_panel_context_menu(ui: &mut Ui) -> Option<std::path::PathBuf> {
        let mut result = None;
        if ui.button("Add existing data source").clicked() {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("Excel Files", &["xlsx"])
                .add_filter("CSV Files", &["csv"])
                .pick_file() 
            {
                result = Some(path);
            }
            ui.close();
        }
        if ui.button("Add new data source").clicked() {
             if let Some(path) = rfd::FileDialog::new()
                .add_filter("CSV Files", &["csv"])
                .set_file_name("new_data_source.csv")
                .save_file() 
            {
                result = Some(path);
            }
            ui.close();
        }
        result
    }
}
