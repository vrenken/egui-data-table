use std::any::Any;
use crate::application_command::ApplicationCommand;

pub struct ClearUserModificationFlag;

impl ApplicationCommand for ClearUserModificationFlag {
    fn as_any(&self) -> &dyn Any { self }
}