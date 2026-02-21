use crate::data::*;
use crate::view::*;

impl DataSource {
    pub fn ui(
        &self,
        ui: &mut egui::Ui,
        ds_idx: usize,
        renaming_target_opt: Option<Rename>,
        view_model: &mut RootViewModel,
        newly_selected_index: &mut Option<usize>,
        newly_selected_sheet_index: &mut Option<usize>,
    ) {
        let default_file_name = std::path::Path::new(&self.path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown")
            .to_string();

        let ds_display_name = self.name.as_ref().unwrap_or(&default_file_name).clone();

        let icon = self
            .sheets
            .first()
            .map(|s| s.icon)
            .unwrap_or(egui_material_icons::icons::ICON_TABLE_CHART);

        let renaming_target_id = egui::Id::new("renaming_target");

        if self.sheets.len() > 1 {
            let mut header = egui::CollapsingHeader::new(format!("{} {}", icon, ds_display_name))
                .default_open(true);

            let renaming_this_ds =
                renaming_target_opt.map_or(false, |t| t == Rename::DataSource(ds_idx));

            if renaming_this_ds {
                header = egui::CollapsingHeader::new(format!("{} ", icon));
            }

            let header_res = header.show(ui, |ui| {
                for sheet_idx in 0..self.sheets.len() {
                    let sheet = &self.sheets[sheet_idx];
                    let selected = view_model.selected_index == Some(ds_idx)
                        && self.selected_sheet_index == sheet_idx;
                    let renaming_target_opt =
                        ui.data(|d| d.get_temp::<Rename>(renaming_target_id));
                    let renaming_this_sheet =
                        renaming_target_opt.map_or(false, |t| t == Rename::Sheet(ds_idx, sheet_idx));

                    let sheet_display_name = sheet
                        .display_name
                        .as_ref()
                        .unwrap_or(&sheet.name)
                        .clone();

                    if renaming_this_sheet {
                        Project::ui_item_as_editable(
                            ui,
                            view_model,
                            Rename::Sheet(ds_idx, sheet_idx),
                            ui.id().with("rename_sheet"),
                            "  📄",
                            &sheet_display_name,
                        );
                    } else {
                        Project::ui_item_as_selectable(
                            ui,
                            Rename::Sheet(ds_idx, sheet_idx),
                            selected,
                            "  📄",
                            &sheet_display_name,
                            Some(&self.path),
                            || {
                                if view_model.selected_index != Some(ds_idx)
                                    || self.selected_sheet_index != sheet_idx
                                {
                                    *newly_selected_index = Some(ds_idx);
                                    *newly_selected_sheet_index = Some(sheet_idx);
                                }
                            },
                        );
                    }
                }
            });

            if renaming_this_ds {
                let mut rect = header_res.header_response.rect;
                rect.min.x += 20.0; // Offset for icon
                header_res.header_response.context_menu(|ui| {
                    Project::ui_item_context_menu(ui, Rename::DataSource(ds_idx));
                });
                ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
                    Project::ui_item_as_editable(
                        ui,
                        view_model,
                        Rename::DataSource(ds_idx),
                        ui.id().with("rename_ds"),
                        "",
                        &ds_display_name,
                    );
                });
            } else {
                header_res.header_response.context_menu(|ui| {
                    Project::ui_item_context_menu(ui, Rename::DataSource(ds_idx));
                });

                if header_res.header_response.clicked() {
                    if view_model.selected_index != Some(ds_idx) {
                        *newly_selected_index = Some(ds_idx);
                        *newly_selected_sheet_index = Some(self.selected_sheet_index);
                    }
                }
                if header_res.header_response.double_clicked() {
                    ui.data_mut(|d| d.insert_temp(renaming_target_id, Rename::DataSource(ds_idx)));
                }
            }
        } else {
            let selected = view_model.selected_index == Some(ds_idx);
            let renaming_this_ds =
                renaming_target_opt.map_or(false, |t| t == Rename::DataSource(ds_idx));

            if renaming_this_ds {
                Project::ui_item_as_editable(
                    ui,
                    view_model,
                    Rename::DataSource(ds_idx),
                    ui.id().with("rename_ds"),
                    icon,
                    &ds_display_name,
                );
            } else {
                Project::ui_item_as_selectable(
                    ui,
                    Rename::DataSource(ds_idx),
                    selected,
                    icon,
                    &ds_display_name,
                    Some(&self.path),
                    || {
                        if view_model.selected_index != Some(ds_idx) {
                            *newly_selected_index = Some(ds_idx);
                            *newly_selected_sheet_index = Some(0);
                        }
                    },
                );
            }
        }
    }
}