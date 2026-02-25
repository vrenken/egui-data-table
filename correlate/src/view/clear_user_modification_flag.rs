use std::any::Any;
use crate::application_command::*;
use crate::view::RootViewModel;


pub struct ClearUserModificationFlag {
    pub ctx: egui::Context,
}
impl ApplicationCommand for ClearUserModificationFlag {
    fn as_any(&self) -> &dyn Any { self }
}

pub struct ClearUserModificationFlagHandler;
impl ApplicationCommandHandler for ClearUserModificationFlagHandler {
    fn handle(&self, cmd: &dyn Any) {
        if let Some(command) = cmd.downcast_ref::<ClearUserModificationFlag>() {
            let view_model_ptr = command.ctx.data(|d| d.get_temp::<usize>(egui::Id::new("root_view_model"))).unwrap();
            let view_model = unsafe { &mut *(view_model_ptr as *mut RootViewModel) };

            view_model.table.clear_user_modification_flag();
            view_model.save_datasource_configuration();
        }
    }
}
