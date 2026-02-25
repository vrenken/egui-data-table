use std::any::Any;
use egui::{Context, Id};
use crate::application_command::*;
use crate::view::RootViewModel;

pub struct AddProject {
    pub ctx: Context,
}
impl ApplicationCommand for AddProject {
    fn as_any(&self) -> &dyn Any { self }
}

pub struct AddProjectHandler;
impl ApplicationCommandHandler for AddProjectHandler {
    fn handle(&self, cmd: &dyn Any) {
        if let Some(command) = cmd.downcast_ref::<AddProject>() {
            let view_model_ptr = command.ctx.data(|d| d.get_temp::<usize>(Id::new("root_view_model"))).unwrap();
            let view_model = unsafe { &mut *(view_model_ptr as *mut RootViewModel) };
            view_model.add_project();
        }
    }
}
