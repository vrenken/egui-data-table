use std::borrow::Cow;
use egui::{Response, Key};
use egui_data_table::RowViewer;
use egui_data_table::viewer::{default_hotkeys, CellWriteContext, RowCodec, UiActionContext};
use crate::data::*;
use crate::data::column_config::ColumnConfig;
use crate::data::column_type::ColumnType;
use crate::data::RenamingTarget;

pub struct RowView {
    pub name_filter: String,
    pub row_protection: bool,
    pub hotkeys: Vec<(egui::KeyboardShortcut, egui_data_table::UiAction)>,
    pub column_configs: Vec<ColumnConfig>,
}

impl RowViewer<Row> for RowView {
    fn num_columns(&mut self) -> usize {
        self.column_configs.len()
    }

    fn column_name(&mut self, column: usize) -> Cow<'static, str> {
        self.column_configs.get(column)
            .map(|c| {
                let mut name = c.display_name.as_ref().unwrap_or(&c.name).clone();
                if c.is_key {
                    name = format!("{} {}", egui_material_icons::icons::ICON_KEY, name);
                }
                if c.is_name {
                    name = format!("{} {}", egui_material_icons::icons::ICON_VISIBILITY, name);
                }
                if c.is_virtual {
                    name = format!("{} {}", egui_material_icons::icons::ICON_SYRINGE, name);
                }

                let type_icon = c.column_type.icon();
                name = format!("{} {}", type_icon, name);

                Cow::Owned(name)
            })
            .unwrap_or_else(|| Cow::Owned(format!("Column {}", column)))
    }

    fn try_create_codec(&mut self, _: bool) -> Option<impl RowCodec<Row>> {
        Some(crate::codec::Codec { column_configs: self.column_configs.clone() })
    }

    fn column_render_config(
        &mut self,
        column: usize,
        is_last_visible_column: bool,
    ) -> egui_data_table::viewer::TableColumnConfig {
        let mut config = if is_last_visible_column {
            egui_data_table::viewer::TableColumnConfig::remainder().at_least(24.0)
        } else {
            egui_data_table::viewer::TableColumnConfig::auto().resizable(true)
        };

        if let Some(col_config) = self.column_configs.get(column) {
            if let Some(width) = col_config.width {
                config = egui_data_table::viewer::TableColumnConfig::initial(width).resizable(true);
                if is_last_visible_column {
                    config = config.at_least(24.0);
                }
            }
        }
        config
    }

    fn is_sortable_column(&mut self, _: usize) -> bool {
        true
    }

    fn is_editable_cell(&mut self, column: usize, _row: usize, row_value: &Row) -> bool {
        // We still need a way to identify the "Row locked" column if it exists.
        // For now, let's see if we can find it by name or type if it's special.
        // In the original it was ROW_LOCKED = 5.
        
        let row_locked = self.column_configs.iter().enumerate().find(|(_, c)| c.name == "Row locked")
            .and_then(|(idx, _)| {
                row_value.cells[idx].0.parse::<bool>().ok()
            }).unwrap_or(false);

        // allow editing of the locked flag, but prevent editing other columns when locked.
        if let Some(config) = self.column_configs.get(column) {
            if config.name == "Row locked" {
                return true;
            }
        }
        !row_locked
    }

    fn compare_cell(&self, row_l: &Row, row_r: &Row, column: usize) -> std::cmp::Ordering {
        let config = &self.column_configs[column];
        match config.column_type {
            ColumnType::Number => {
                let l: f64 = row_l.cells[column].0.parse().unwrap_or(0.0);
                let r: f64 = row_r.cells[column].0.parse().unwrap_or(0.0);
                l.partial_cmp(&r).unwrap_or(std::cmp::Ordering::Equal)
            }
            _ => row_l.cells[column].0.cmp(&row_r.cells[column].0),
        }
    }

    fn row_filter_hash(&mut self) -> &impl std::hash::Hash {
        &self.name_filter
    }

    fn filter_row(&mut self, row: &Row) -> bool {
        // filter by the first string column found, or "Name"
        let name_idx = self.column_configs.iter().position(|c| c.name.contains("Name"))
            .or_else(|| self.column_configs.iter().position(|c| c.column_type == ColumnType::Text))
            .unwrap_or(0);

        if let Some(cell) = row.cells.get(name_idx) {
            cell.to_string().contains(&self.name_filter)
        } else {
            false
        }
    }

    fn show_cell_view(&mut self, ui: &mut egui::Ui, row: &Row, column: usize) {
        if let Some(config) = self.column_configs.get_mut(column) {
            config.width = Some(ui.available_width());
        }

        let cell = &row.cells[column];
        let resp = match self.column_configs[column].column_type {
            ColumnType::Bool => {
                let mut b = cell.0.parse::<bool>().unwrap_or(false);
                ui.checkbox(&mut b, "")
            }
            _ => ui.label(&cell.0),
        };

        if let Some(config) = self.column_configs.get(column) {
            if config.is_key {
                ui.painter().rect_filled(
                    resp.rect.expand(2.0),
                    egui::CornerRadius::ZERO,
                    ui.visuals().selection.bg_fill.gamma_multiply(0.1),
                );
            }
        }

        resp.context_menu(|ui| {
            crate::view::central_panel::CentralPanel::ui_row_context_menu(self, ui, column);
        });
    }

    fn on_cell_view_response(
        &mut self,
        _row: &Row,
        _column: usize,
        resp: &Response,
    ) -> Option<Box<Row>> {
        resp.dnd_release_payload::<String>()
            .map(|x| {
                let mut cells = Vec::with_capacity(self.column_configs.len());
                for config in &self.column_configs {
                    let cell = match config.column_type {
                        ColumnType::Text => CellValue((*x).clone()),
                        ColumnType::Number => config.column_type.default_value(),
                        ColumnType::DateTime => config.column_type.default_value(),
                        ColumnType::Bool => config.column_type.default_value(),
                        ColumnType::Select => CellValue((*x).clone()),
                        ColumnType::MultiSelect => CellValue((*x).clone()),
                    };
                    cells.push(cell);
                }
                Box::new(Row { cells })
            })
    }

    fn show_cell_editor(
        &mut self,
        ui: &mut egui::Ui,
        row: &mut Row,
        column: usize,
    ) -> Option<Response> {
        let column_config = self.column_configs.get_mut(column)?;
        let column_type = column_config.column_type;
        let cell_value = &mut row.cells[column];

        column_type.show_editor(ui, cell_value, column_config)
    }

    fn set_cell_value(&mut self, src: &Row, dst: &mut Row, column: usize) {
        dst.cells[column] = src.cells[column].clone();
    }

    fn confirm_cell_write_by_ui(
        &mut self,
        current: &Row,
        _next: &Row,
        _column: usize,
        _context: CellWriteContext,
    ) -> bool {
        if !self.row_protection {
            return true;
        }

        let is_student_idx = self.column_configs.iter().position(|c| c.name == "Is Student (Not sortable)");
        if let Some(idx) = is_student_idx {
            if let Ok(is_student) = current.cells[idx].0.parse::<bool>() {
                return !is_student;
            }
        }
        true
    }

    fn confirm_row_deletion_by_ui(&mut self, row: &Row) -> bool {
        if !self.row_protection {
            return true;
        }

        let is_student_idx = self.column_configs.iter().position(|c| c.name == "Is Student (Not sortable)");
        if let Some(idx) = is_student_idx {
            if let Ok(is_student) = row.cells[idx].0.parse::<bool>() {
                return !is_student;
            }
        }
        true
    }

    fn new_empty_row(&mut self) -> Row {
        let cells = self.column_configs.iter()
            .map(|config| config.column_type.default_value())
            .collect();
        Row { cells }
    }

    fn on_highlight_cell(&mut self, row: &Row, column: usize) {
        println!("cell highlighted: row: {:?}, column: {}", row, column);
    }

    fn on_highlight_change(&mut self, highlighted: &[&Row], unhighlighted: &[&Row]) {
        println!("highlight {:?}", highlighted);
        println!("unhighlight {:?}", unhighlighted);
    }

    fn on_row_updated(&mut self, row_index: usize, new_row: &Row, old_row: &Row) {
        println!("row updated. row_id: {}, new_row: {:?}, old_row: {:?}", row_index, new_row, old_row);
    }

    fn on_row_inserted(&mut self, row_index: usize, row: &Row) {
        println!("row inserted. row_id: {}, values: {:?}", row_index, row);
    }

    fn on_row_removed(&mut self, row_index: usize, row: &Row) {
        println!("row removed. row_id: {}, values: {:?}", row_index, row);
    }

    fn on_rename_committed(&mut self, table: &mut egui_data_table::DataTable<Row>, target: egui_data_table::viewer::RenameTarget, new_name: String) {
        let renaming_target = match target {
            egui_data_table::viewer::RenameTarget::Row(idx) => Some(RenamingTarget::Row(idx)),
            egui_data_table::viewer::RenameTarget::Column(idx) => Some(RenamingTarget::Column(idx)),
            _ => None,
        };

        if let Some(renaming_target) = renaming_target {
            match renaming_target {
                RenamingTarget::Row(row_idx) => {
                    let name_col_idx = self.column_configs.iter().position(|c| c.is_name)
                        .or_else(|| self.column_configs.iter().position(|c| c.name.contains("Name")))
                        .or_else(|| self.column_configs.iter().position(|c| c.column_type == crate::data::ColumnType::Text))
                        .unwrap_or(0);

                    if let Some(row) = table.get_mut(row_idx) {
                        let column_type = self.column_configs[name_col_idx].column_type;
                        row.cells[name_col_idx] = map_cell_value(&new_name, column_type);
                        table.mark_as_modified();
                    }
                }
                RenamingTarget::Column(col_idx) => {
                    if let Some(config) = self.column_configs.get_mut(col_idx) {
                        config.display_name = if new_name.is_empty() || new_name == config.name { None } else { Some(new_name) };
                        if config.is_virtual {
                            config.name = config.display_name.clone().unwrap_or_else(|| config.name.clone());
                        }
                        table.mark_as_modified();
                    }
                }
                _ => {}
            }
        }
    }

    fn on_column_inserted(&mut self, table: &mut egui_data_table::DataTable<Row>, at: usize) {
        let new_column = crate::data::ColumnConfig {
            name: format!("New Column {}", self.column_configs.len() + 1),
            display_name: None,
            column_type: crate::data::ColumnType::Text,
            is_key: false,
            is_name: false,
            is_virtual: true,
            order: self.column_configs.len(),
            width: None,
            allowed_values: None,
        };
        self.column_configs.insert(at, new_column);
        // Update all rows in the table
        let mut rows = table.take();
        for row in &mut rows {
            row.cells.insert(at, self.column_configs[at].column_type.default_value());
        }
        table.replace(rows);
        table.mark_as_modified();
    }

    fn column_header_context_menu(&mut self, ui: &mut egui::Ui, column: usize) -> egui_data_table::viewer::HeaderResult {
        let mut action = None;
        ui.horizontal(|ui| {
            ui.label(egui_material_icons::icons::ICON_NOTES);

            let config = &self.column_configs[column];
            let initial_name = config.display_name.as_ref().unwrap_or(&config.name).clone();

            let mut current_name = ui.data_mut(|d| d.get_temp::<String>(ui.id().with("rename")).unwrap_or(initial_name));

            let res = ui.text_edit_singleline(&mut current_name);
            if res.lost_focus() || ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                action = Some(egui_data_table::viewer::HeaderAction::RenameCommitted(current_name.clone()));
                ui.data_mut(|d| d.remove::<String>(ui.id().with("rename")));
                ui.close();
            } else if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                ui.data_mut(|d| d.remove::<String>(ui.id().with("rename")));
                ui.close();
            } else {
                ui.data_mut(|d| d.insert_temp(ui.id().with("rename"), current_name));
            }
            res.request_focus();

        });

        ui.menu_button(format!("{} Change type", egui_material_icons::icons::ICON_EDIT_SQUARE), |ui| {
            let current_type = self.column_configs[column].column_type;

            let mut is_text = current_type == ColumnType::Text;
            if ui.checkbox(&mut is_text, format!("{} Text", ColumnType::Text.icon())).clicked() {
                self.column_configs[column].column_type = ColumnType::Text;
                action = Some(egui_data_table::viewer::HeaderAction::RequestSave);
                ui.close();
            }
            let mut is_number = current_type == ColumnType::Number;
            if ui.checkbox(&mut is_number, format!("{} Number", ColumnType::Number.icon())).clicked() {
                self.column_configs[column].column_type = ColumnType::Number;
                action = Some(egui_data_table::viewer::HeaderAction::RequestSave);
                ui.close();
            }
            let mut is_date_time = current_type == ColumnType::DateTime;
            if ui.checkbox(&mut is_date_time, format!("{} Date / time", ColumnType::DateTime.icon())).clicked() {
                self.column_configs[column].column_type = ColumnType::DateTime;
                action = Some(egui_data_table::viewer::HeaderAction::RequestSave);
                ui.close();
            }

            let is_virtual = self.column_configs[column].is_virtual;
            ui.add_enabled_ui(is_virtual, |ui| {
                let mut is_select = current_type == ColumnType::Select;
                if ui.checkbox(&mut is_select, format!("{} Select", ColumnType::Select.icon())).clicked() {
                    self.column_configs[column].column_type = ColumnType::Select;
                    action = Some(egui_data_table::viewer::HeaderAction::RequestSave);
                    ui.close();
                }
                let mut is_multi_select = current_type == ColumnType::MultiSelect;
                if ui.checkbox(&mut is_multi_select, format!("{} Multi-select", ColumnType::MultiSelect.icon())).clicked() {
                    self.column_configs[column].column_type = ColumnType::MultiSelect;
                    action = Some(egui_data_table::viewer::HeaderAction::RequestSave);
                    ui.close();
                }
            });

            ui.add_enabled_ui(false, |ui| {
                if ui.button(format!("{} Status", egui_material_icons::icons::ICON_TARGET)).clicked() {
                    ui.close();
                }
                if ui.button(format!("{} Person", egui_material_icons::icons::ICON_GROUP)).clicked() {
                    ui.close();
                }
                if ui.button(format!("{} URL", egui_material_icons::icons::ICON_LINK)).clicked() {
                    ui.close();
                }
                if ui.button(format!("{} Email", egui_material_icons::icons::ICON_ALTERNATE_EMAIL)).clicked() {
                    ui.close();
                }
                if ui.button(format!("{} Phone", egui_material_icons::icons::ICON_CALL)).clicked() {
                    ui.close();
                }
                if ui.button(format!("{} Relation", egui_material_icons::icons::ICON_NORTH_EAST)).clicked() {
                    ui.close();
                }
                if ui.button(format!("{} Rollup", egui_material_icons::icons::ICON_SEARCH)).clicked() {
                    ui.close();
                }
                if ui.button(format!("{} Created time", egui_material_icons::icons::ICON_NEST_CLOCK_FARSIGHT_ANALOG)).clicked() {
                    ui.close();
                }
                if ui.button(format!("{} Created by", egui_material_icons::icons::ICON_ACCOUNT_CIRCLE)).clicked() {
                    ui.close();
                }
                if ui.button(format!("{} Last edited time", egui_material_icons::icons::ICON_NEST_CLOCK_FARSIGHT_ANALOG)).clicked() {
                    ui.close();
                }
                if ui.button(format!("{} Last edited by", egui_material_icons::icons::ICON_ACCOUNT_CIRCLE)).clicked() {
                    ui.close();
                }
                if ui.button(format!("{} Location", egui_material_icons::icons::ICON_LOCATION_ON)).clicked() {
                    ui.close();
                }
            });

        });

        let is_name_active = self.column_configs[column].is_name;
        let is_key_active = self.column_configs[column].is_key;

        ui.separator(); // ========================================

        if ui.button(format!("{} Filter", egui_material_icons::icons::ICON_FILTER_LIST)).clicked() {
            ui.close();
        }
        ui.menu_button(format!("{} Sort", egui_material_icons::icons::ICON_SWAP_VERT), |ui| {
            if ui.button(format!("{} Sort ascending", egui_material_icons::icons::ICON_NORTH)).clicked() {
                ui.close();
            }
            if ui.button(format!("{} Sort descending", egui_material_icons::icons::ICON_SOUTH)).clicked() {
                ui.close();
            }

        });
        if ui.button(format!("{} Hide", egui_material_icons::icons::ICON_VISIBILITY_OFF)).clicked() {
            ui.close();
        }

        let mut is_key = is_key_active;
        if ui.checkbox(&mut is_key, "Use as key").clicked() {
            self.column_configs[column].is_key = is_key;
            action = Some(egui_data_table::viewer::HeaderAction::RequestSave);
            ui.close();
        }

        let mut is_name = is_name_active;
        if ui.checkbox(&mut is_name, "Use as name").clicked() {
            if is_name {
                // Turn off is_name for all other columns
                for c in self.column_configs.iter_mut() {
                    c.is_name = false;
                }
                self.column_configs[column].is_name = true;
            } else {
                self.column_configs[column].is_name = false;
            }
            action = Some(egui_data_table::viewer::HeaderAction::RequestSave);
            ui.close();
        }
        ui.separator();

        if ui.button(format!("{} Insert left", egui_material_icons::icons::ICON_ADD_COLUMN_LEFT)).clicked() {
            action = Some(egui_data_table::viewer::HeaderAction::AddColumn(column));
            ui.close();
        }
        if ui.button(format!("{} Insert right", egui_material_icons::icons::ICON_ADD_COLUMN_RIGHT)).clicked() {
            action = Some(egui_data_table::viewer::HeaderAction::AddColumn(column + 1));
            ui.close();
        }


        ui.separator();

        if column > 0 {
            if ui.button("Move Left").clicked() {
                self.column_configs.swap(column, column - 1);
                action = Some(egui_data_table::viewer::HeaderAction::RequestSave);
                ui.close();
            }
        }
        if column < self.column_configs.len() - 1 {
            if ui.button("Move Right").clicked() {
                self.column_configs.swap(column, column + 1);
                action = Some(egui_data_table::viewer::HeaderAction::RequestSave);
                ui.close();
            }
        }

        ui.separator();

        if ui.button(format!("{} Duplicate", egui_material_icons::icons::ICON_STACK)).clicked() {
            ui.close();
        }
        if ui.button(format!("{} Trash", egui_material_icons::icons::ICON_DELETE)).clicked() {
            ui.close();
        }

        action
    }

    fn row_header_double_clicked(&mut self, ctx: &egui::Context, row_idx: usize, _row: &Row) {
        ctx.data_mut(|d| d.insert_temp(egui::Id::new("renaming_target"), RenamingTarget::Row(row_idx)));
    }

    fn column_header_double_clicked(&mut self, ctx: &egui::Context, column: usize) {
        ctx.data_mut(|d| d.insert_temp(egui::Id::new("renaming_target"), RenamingTarget::Column(column)));
    }

    fn show_column_header(&mut self, ui: &mut egui::Ui, column: usize) {
        ui.add(egui::Label::new(self.column_name(column)).selectable(false));
    }

    fn show_row_header(&mut self, ui: &mut egui::Ui, row_idx: usize, vis_row: usize, has_any_sort: bool, row_id_digits: usize, vis_row_digits: usize, row: &Row) -> Option<(egui_data_table::viewer::RenameTarget, String)> {
        let renaming_target = ui.data(|d| d.get_temp::<RenamingTarget>(egui::Id::new("renaming_target")));
        let renaming_this_row = renaming_target.map_or(false, |t| t == RenamingTarget::Row(row_idx));
        let mut committed = None;

        if renaming_this_row {
            let name_col_idx = self.column_configs.iter().position(|c| c.is_name)
                .or_else(|| self.column_configs.iter().position(|c| c.name.contains("Name")))
                .or_else(|| self.column_configs.iter().position(|c| c.column_type == crate::data::ColumnType::Text))
                .unwrap_or(0);

            let initial_name = row.cells[name_col_idx].0.clone();

            let mut current_name = ui.data_mut(|d| d.get_temp::<String>(ui.id().with("rename")).unwrap_or(initial_name));

            let res = ui.text_edit_singleline(&mut current_name);
            if res.lost_focus() || ui.input(|i| i.key_pressed(Key::Enter)) {
                committed = Some((egui_data_table::viewer::RenameTarget::Row(row_idx), current_name.clone()));
                ui.data_mut(|d| {
                    d.remove::<RenamingTarget>(egui::Id::new("renaming_target"));
                    d.remove::<String>(ui.id().with("rename"));
                });
            } else if ui.input(|i| i.key_pressed(Key::Escape)) {
                ui.data_mut(|d| {
                    d.remove::<RenamingTarget>(egui::Id::new("renaming_target"));
                    d.remove::<String>(ui.id().with("rename"));
                });
            } else {
                ui.data_mut(|d| d.insert_temp(ui.id().with("rename"), current_name));
            }
            res.request_focus();
        } else {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.separator();

                if has_any_sort {
                    ui.monospace(
                        egui::RichText::from(format!(
                            "{:·>width$}",
                            row_idx,
                            width = row_id_digits
                        ))
                        .strong(),
                    );
                } else {
                    ui.monospace(
                        egui::RichText::from(format!("{:>width$}", "", width = row_id_digits))
                            .strong(),
                    );
                }

                ui.monospace(
                    egui::RichText::from(format!(
                        "{:·>width$}",
                        vis_row + 1,
                        width = vis_row_digits
                    ))
                    .weak(),
                );
            });
        }
        committed
    }

    fn hotkeys(
        &mut self,
        context: &UiActionContext,
    ) -> Vec<(egui::KeyboardShortcut, egui_data_table::UiAction)> {
        let hotkeys = default_hotkeys(context);
        self.hotkeys.clone_from(&hotkeys);
        hotkeys
    }

    fn persist_ui_state(&self) -> bool {
        true
    }
}
