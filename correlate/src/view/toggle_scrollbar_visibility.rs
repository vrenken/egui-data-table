use std::any::Any;
use crate::application_command::ApplicationCommand;

pub struct ToggleScrollBarVisibility;

impl ApplicationCommand for ToggleScrollBarVisibility {
    fn as_any(&self) -> &dyn Any { self }
}