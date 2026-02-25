use std::any::Any;
use egui::{Context, Id};
use crate::application_command::*;
use crate::view::RootViewModel;

pub struct AddExistingDataSource {
    pub ctx: Context,
    pub path: std::path::PathBuf,
}
impl ApplicationCommand for AddExistingDataSource {
    fn as_any(&self) -> &dyn Any { self }
}

pub struct AddExistingDataSourceHandler;
impl ApplicationCommandHandler for AddExistingDataSourceHandler {
    fn handle(&self, cmd: &dyn Any) {
        if let Some(command) = cmd.downcast_ref::<AddExistingDataSource>() {
            let view_model_ptr = command.ctx.data(|d| d.get_temp::<usize>(Id::new("root_view_model"))).unwrap();
            let view_model = unsafe { &mut *(view_model_ptr as *mut RootViewModel) };
            view_model.handle_pending_file_add(command.path.clone(), 0);
        }
    }
}