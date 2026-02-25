use std::any::Any;

pub trait ApplicationCommand: Any + Send + Sync {
    fn as_any(&self) -> &dyn Any;
}