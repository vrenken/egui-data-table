use egui::Id;
use egui::Ui;
use egui::Context;
use std::any::{Any, TypeId};
use std::collections::HashMap;


pub trait ApplicationCommand: Any + Send + Sync {
    fn as_any(&self) -> &dyn Any;
}

pub trait ApplicationCommandHandler {
    fn handle(&self, cmd: &dyn Any);
}


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


pub fn enqueue_ui_command(ui: &mut Ui, command: Box<dyn ApplicationCommand>) {
    let key = Id::new("ui_commands");

    ui.data_mut(|data| {
        let list = data.get_temp_mut_or_insert_with::<UICommands>(key, UICommands::default);
        list.0.push(command);
    });
}


pub fn get_commands(ctx: &Context) -> Vec<Box<dyn ApplicationCommand>> {
    let key = Id::new("ui_commands");

    ctx.data_mut(|data| {
        let list = data.get_temp_mut_or_insert_with::<UICommands>(key, UICommands::default);
        std::mem::take(&mut list.0)
    })
}

#[derive(Default)]
struct UICommands(Vec<Box<dyn ApplicationCommand>>);

impl Clone for UICommands {
    fn clone(&self) -> Self {
        panic!("UICommands should never be cloned");
    }
}


