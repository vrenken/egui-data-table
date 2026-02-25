use std::any::Any;
use egui::scroll_area::ScrollBarVisibility;
use crate::application_command::*;
use crate::view::RootViewModel;


pub struct ToggleScrollBarVisibility {
    pub ctx: egui::Context,
}
impl ApplicationCommand for ToggleScrollBarVisibility {
    fn as_any(&self) -> &dyn Any { self }
}

pub struct ToggleScrollBarVisibilityHandler;
impl ApplicationCommandHandler for ToggleScrollBarVisibilityHandler {
    fn handle(&self, cmd: &dyn Any) {
        if let Some(command) = cmd.downcast_ref::<ToggleScrollBarVisibility>() {
            let view_model_ptr = command.ctx.data(|d| d.get_temp::<usize>(egui::Id::new("root_view_model"))).unwrap();
            let view_model = unsafe { &mut *(view_model_ptr as *mut RootViewModel) };

            view_model.scroll_bar_always_visible = !view_model.scroll_bar_always_visible;
            if view_model.scroll_bar_always_visible {
                view_model.style_override.scroll_bar_visibility = ScrollBarVisibility::AlwaysVisible;
            } else {
                view_model.style_override.scroll_bar_visibility = ScrollBarVisibility::VisibleWhenNeeded;
            }
        }
    }
}