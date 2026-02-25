use std::any::Any;

trait ApplicationCommand: Any + Send + Sync {}

impl<T: Any + Send + Sync> ApplicationCommand for T {}