use crate::view::*;

pub struct RootView {
    pub(crate) view_model: RootViewModel,
    pub(crate) central_panel: CentralPanel,
    pub(crate) bottom_panel: BottomPanel,
    pub(crate) menu_bar: MenuBar,
}

impl Default for RootView {

    fn default() -> Self {
        Self {
            view_model: RootViewModel::default(),
            central_panel: CentralPanel::default(),
            bottom_panel: BottomPanel::default(),
            menu_bar: MenuBar::default(),
        }
    }
}

impl eframe::App for RootView {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Assert Send/Sync for DataTable as a compile-time check
        fn is_send<T: Send>(_: &T) {}
        fn is_sync<T: Sync>(_: &T) {}
        is_send(&self.view_model.table);
        is_sync(&self.view_model.table);

        self.menu_bar.ui(&mut self.view_model, ctx);
        self.bottom_panel.ui(&mut self.view_model, ctx);

        let (newly_selected_index, newly_selected_sheet_index): (Option<usize>, Option<usize>) = HierarchyPanel::default().ui(&mut self.view_model, ctx);

        if let Some(index) = newly_selected_index {
            let sheet_idx = newly_selected_sheet_index.unwrap_or(0);
            self.switch_to_source(index, sheet_idx);
        }

        self.handle_pending_file_add();
        self.central_panel.ui(&mut self.view_model, ctx);

        if let Some(index) = self.view_model.save_requested.take() {
            self.view_model.save_source_config(index);
        }
    }
}