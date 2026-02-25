use crate::view::*;
use crate::application_command::*;

pub struct RootView {
    pub root_view_model: RootViewModel,
    #[allow(dead_code)] // TODO: Validate
    pub hierarchy_view_model: HierarchyViewModel,
    pub central_panel_view_model: CentralPanelViewModel,

    pub central_panel: CentralPanel,
    pub bottom_panel: BottomPanel,
    pub menu_bar: MenuBar,
    pub hierarchy_panel: HierarchyPanel,

    pub pending_commands: Vec<Box<dyn ApplicationCommand>>,
    pub command_dispatcher: ApplicationCommandDispatcher,
}

impl Default for RootView {

    fn default() -> Self {

        let config_path = "config.json";
        let config = crate::data::Configuration::load(config_path).unwrap();

        let dispatcher = ApplicationCommandDispatcher::new();
        //dispatcher.register()

        Self {
            hierarchy_view_model: HierarchyViewModel::default(&config),
            central_panel_view_model: CentralPanelViewModel::default(&config),
            root_view_model: RootViewModel::default(config),
            central_panel: CentralPanel::default(),
            bottom_panel: BottomPanel::default(),
            menu_bar: MenuBar::default(),
            hierarchy_panel: HierarchyPanel::default(),
            pending_commands: Vec::new(),
            command_dispatcher: dispatcher,
        }
    }
}

impl eframe::App for RootView {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        let commands = self.pending_commands.drain(..).collect::<Vec<_>>();
        self.hierarchy_panel.update(&mut self.root_view_model, ctx, &commands);
        self.central_panel.update(
            &mut self.root_view_model,
            &mut self.central_panel_view_model,
            ctx,
            &commands
        );

        // Assert Send/Sync for DataTable as a compile-time check
        fn is_send<T: Send>(_: &T) {}
        fn is_sync<T: Sync>(_: &T) {}
        is_send(&self.root_view_model.table);
        is_sync(&self.root_view_model.table);

        self.menu_bar.ui(&mut self.root_view_model, ctx);
        self.bottom_panel.ui(&mut self.root_view_model, ctx);

        self.pending_commands.extend(self.hierarchy_panel.ui(&mut self.root_view_model, ctx));
        self.pending_commands.extend(self.central_panel.ui(&mut self.root_view_model, ctx));
    }
}