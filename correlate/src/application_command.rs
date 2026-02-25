pub trait ApplicationCommand: Any + Send + Sync {
    fn as_any(&self) -> &dyn Any;
}

pub trait ApplicationCommandHandler {
    fn handle(&self, cmd: &dyn Any);
}

use std::any::{Any, TypeId};
use std::collections::HashMap;


pub struct ApplicationCommandDispatcher {
    handlers: HashMap<TypeId, Box<dyn ApplicationCommandHandler>>,
}

impl ApplicationCommandDispatcher {
    pub fn new() -> Self {
        Self { handlers: HashMap::new() }
    }

    pub fn register<C: Any + Send + Sync, H: ApplicationCommandHandler + 'static>(&mut self, handler: H) {
        self.handlers.insert(TypeId::of::<C>(), Box::new(handler));
    }

    pub fn dispatch(&self, cmd: &dyn Any) {
        if let Some(handler) = self.handlers.get(&cmd.type_id()) {
            handler.handle(cmd);
        }
    }
}
