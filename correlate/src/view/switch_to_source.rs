use std::any::Any;
use egui::{Context, Id};
use crate::application_command::*;
use crate::view::RootViewModel;


pub struct SwitchToSource {
    pub ctx: Context,
    pub index: usize,
    pub sheet_idx: usize,
}
impl ApplicationCommand for SwitchToSource {
    fn as_any(&self) -> &dyn Any { self }
}

pub struct SwitchToSourceHandler;
impl ApplicationCommandHandler for SwitchToSourceHandler {
    fn handle(&self, cmd: &dyn Any) {
        if let Some(command) = cmd.downcast_ref::<SwitchToSource>() {
            let view_model_ptr = command.ctx.data(|d| d.get_temp::<usize>(Id::new("root_view_model"))).unwrap();
            let view_model = unsafe { &mut *(view_model_ptr as *mut RootViewModel) };
            view_model.switch_to_source(command.index, command.sheet_idx);
        }
    }
}