use crate::view::*;

pub struct RootView {
    pub root_view_model: RootViewModel,
    pub hierarchy_view_model: HierarchyViewModel,
    pub central_panel_view_model: CentralPanelViewModel,

    pub central_panel: CentralPanel,
    pub bottom_panel: BottomPanel,
    pub menu_bar: MenuBar,
    pub hierarchyPanel: HierarchyPanel,
}

impl Default for RootView {

    fn default() -> Self {

        let config_path = "config.json";
        let config = crate::data::Configuration::load(config_path).unwrap();

        Self {
            hierarchy_view_model: HierarchyViewModel::default(&config),
            central_panel_view_model: CentralPanelViewModel::default(&config),
            root_view_model: RootViewModel::default(config),
            central_panel: CentralPanel::default(),
            bottom_panel: BottomPanel::default(),
            menu_bar: MenuBar::default(),
            hierarchyPanel: HierarchyPanel::default(),
        }
    }
}

impl eframe::App for RootView {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Assert Send/Sync for DataTable as a compile-time check
        fn is_send<T: Send>(_: &T) {}
        fn is_sync<T: Sync>(_: &T) {}
        is_send(&self.root_view_model.table);
        is_sync(&self.root_view_model.table);

        self.menu_bar.ui(&mut self.root_view_model, ctx);
        self.bottom_panel.ui(&mut self.root_view_model, ctx);

        let (newly_selected_index, newly_selected_sheet_index): (Option<usize>, Option<usize>) = self.hierarchyPanel.ui(&mut self.root_view_model, ctx);

        if let Some(index) = newly_selected_index {
            let sheet_idx = newly_selected_sheet_index.unwrap_or(0);
            self.root_view_model.switch_to_source(index, sheet_idx);
        }

        self.central_panel.ui(&mut self.root_view_model, &mut self.central_panel_view_model, ctx);
    }
}