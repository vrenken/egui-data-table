use egui::Sense;
use crate::view::RootViewModel;

#[derive(Default)]
pub struct MenuBar {}

impl MenuBar {
    pub fn ui(&mut self, view_model: &mut RootViewModel, ctx: &egui::Context) {
        egui::TopBottomPanel::top("MenuBar").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.hyperlink_to(
                    " vrenken/etalii-correlate",
                    "https://github.com/vrenken/etalii-correlate",
                );
                
                ui.separator();

                egui::widgets::global_theme_preference_buttons(ui);

                ui.separator();

                ui.label("Name Filter");
                ui.text_edit_singleline(&mut view_model.viewer.name_filter);

                ui.add(egui::Button::new("Drag me and drop on any cell").sense(Sense::drag()))
                    .on_hover_text(
                        "Dropping this will replace the cell \
                        content with some predefined value.",
                    )
                    .dnd_set_drag_payload(String::from("Hallo~"));

                ui.menu_button("🎌 Flags", |ui| {
                    ui.checkbox(&mut view_model.viewer.row_protection, "Row Protection")
                        .on_hover_text(
                            "If checked, any rows marked won't be deleted or overwritten by UI actions.",
                        );

                    ui.checkbox(
                        &mut view_model.style_override.single_click_edit_mode,
                        "Single Click Edit",
                    )
                    .on_hover_text("If checked, cells will be edited with a single click.");

                    ui.checkbox(
                        &mut view_model.style_override.auto_shrink.x,
                        "Auto-shrink X",
                    );
                    ui.checkbox(
                        &mut view_model.style_override.auto_shrink.y,
                        "Auto-shrink Y",
                    );

                    ui.checkbox(
                        &mut view_model.scroll_bar_always_visible,
                        "Scrollbar always visible",
                    );

                    if ui.button("Shuffle Rows").clicked() {
                        fastrand::shuffle(&mut view_model.table);
                    }
                })
            })
        });
    }
}
