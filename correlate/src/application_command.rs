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

    pub fn register<C: ApplicationCommand + 'static, H: ApplicationCommandHandler + 'static>(&mut self, handler: H) {
        self.handlers.insert(TypeId::of::<C>(), Box::new(handler));
    }

    pub fn dispatch(&self, commands: &mut Vec<Box<dyn ApplicationCommand>>) {

        let all_commands = commands.drain(..).collect::<Vec<_>>();
        for command in all_commands {
            self.dispatch_single(command.as_any());
        }
    }

    pub fn dispatch_single(&self, command: &dyn Any) {
        let type_id = command.type_id();

        if let Some(handler) = self.handlers.get(&type_id) {
            handler.handle(command);
        }
    }
}
