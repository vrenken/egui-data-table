use crate::data::*;

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

    pub fn rename(&mut self, new_name: String, config: &mut Configuration) {
        self.configuration.name = new_name;
        if let Err(e) = config.save() {
            log::error!("Failed to save config after project rename: {}", e);
        }
    }
}
