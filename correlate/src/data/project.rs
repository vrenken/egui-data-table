use crate::data::*;
use crate::view::*;
use egui::*;

pub struct Project {
    pub configuration: ProjectConfiguration,
}

impl Project {
    pub fn new(name: String) -> Self {
        Self {
            configuration: ProjectConfiguration {
                name,
                data_sources: vec![],
            },
        }
    }

    pub fn rename(&mut self, new_name: String) {
        self.configuration.name = new_name;
    }
}
