use eframe::emath::Align;
use egui::Layout;
use egui::scroll_area::ScrollBarVisibility;
use crate::view::root_view_model::RootViewModel;
use crate::view::*;

#[derive(Default)]
pub struct CentralPanel {}

impl CentralPanel {
    pub fn ui(&mut self,
              view_model: &mut RootViewModel,
              central_panel_view_model: &mut CentralPanelViewModel,
              ctx: &egui::Context) {

        ctx.data_mut(|d| d.insert_temp(egui::Id::new("root_view_model"), view_model as *mut RootViewModel as usize));

        egui::CentralPanel::default()
            .show(ctx, |ui| {

            ui.vertical(|ui| {

                ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                    if ui.button(egui_material_icons::icons::ICON_PAGE_INFO).clicked() {}
                    if ui.button(egui_material_icons::icons::ICON_SWAP_VERT).clicked() {}
                    if ui.button(egui_material_icons::icons::ICON_FILTER_LIST).clicked() {}
                });

                match view_model.scroll_bar_always_visible {
                    true => {
                        ui.style_mut().spacing.scroll = egui::style::ScrollStyle::solid();
                        view_model.style_override.scroll_bar_visibility = ScrollBarVisibility::AlwaysVisible;
                    },
                    false => {
                        ui.style_mut().spacing.scroll = egui::style::ScrollStyle::floating();
                        view_model.style_override.scroll_bar_visibility = ScrollBarVisibility::VisibleWhenNeeded;
                    }
                };

                //let available = ui.available_size();

                ui.add(
                    //available,
                    egui_data_table::Renderer::new(&mut view_model.table, &mut view_model.viewer).with_style(view_model.style_override),
                );

                if view_model.table.has_user_modification() {
                    view_model.table.clear_user_modification_flag();
                    view_model.save_datasource_configuration();
                }

                central_panel_view_model.handle_viewer_requests(view_model);
            });
        });
    }

    pub fn ui_row_context_menu(viewer: &mut crate::view::RowView, ui: &mut egui::Ui, column: usize) {
        if let Some(config) = viewer.column_configs.get_mut(column) {
            let mut is_key = config.is_key;
            if ui.checkbox(&mut is_key, "Is key").clicked() {
                config.is_key = is_key;
                // Reset the table to force a redraw with new header names
                ui.ctx().memory_mut(|_mem| {
                    // This is a hacky way to force a full redraw of the table
                    // by clearing its UI state cache if we had access to the ID.
                    // Since we don't easily have the ID here, we just hope the change
                    // is picked up on next frame.
                });
                ui.close();
            }
        }
    }
}
