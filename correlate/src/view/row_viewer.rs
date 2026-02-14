use std::borrow::Cow;
use egui::Response;
use egui_data_table::RowViewer;
use egui_data_table::viewer::{default_hotkeys, CellWriteContext, RowCodec, UiActionContext};
use crate::data::*;
use crate::data::column_config::ColumnConfig;
use crate::data::column_type::ColumnType;

pub struct Viewer {
    pub name_filter: String,
    pub row_protection: bool,
    pub hotkeys: Vec<(egui::KeyboardShortcut, egui_data_table::UiAction)>,
    pub column_configs: Vec<ColumnConfig>,
}

impl RowViewer<Row> for Viewer {
    fn num_columns(&mut self) -> usize {
        self.column_configs.len()
    }

    fn column_name(&mut self, column: usize) -> Cow<'static, str> {
        self.column_configs.get(column)
            .map(|c| Cow::Owned(c.name.clone()))
            .unwrap_or_else(|| Cow::Owned(format!("Column {}", column)))
    }

    fn try_create_codec(&mut self, _: bool) -> Option<impl RowCodec<Row>> {
        Some(crate::codec::Codec { column_configs: self.column_configs.clone() })
    }

    fn is_sortable_column(&mut self, column: usize) -> bool {
        self.column_configs.get(column).map(|c| c.is_sortable).unwrap_or(false)
    }

    fn is_editable_cell(&mut self, column: usize, _row: usize, row_value: &Row) -> bool {
        // We still need a way to identify the "Row locked" column if it exists.
        // For now, let's see if we can find it by name or type if it's special.
        // In the original it was ROW_LOCKED = 5.
        
        let row_locked = self.column_configs.iter().enumerate().find(|(_, c)| c.name == "Row locked")
            .and_then(|(idx, _)| {
                if let CellValue::Bool(locked) = row_value.cells[idx] {
                    Some(locked)
                } else {
                    None
                }
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
        match (&row_l.cells[column], &row_r.cells[column]) {
            (CellValue::String(l), CellValue::String(r)) => l.cmp(r),
            (CellValue::Int(l), CellValue::Int(r)) => l.cmp(r),
            (CellValue::Gender(l), CellValue::Gender(r)) => l.cmp(r),
            (CellValue::Bool(l), CellValue::Bool(r)) => l.cmp(r),
            (CellValue::Grade(l), CellValue::Grade(r)) => l.cmp(r),
            _ => std::cmp::Ordering::Equal,
        }
    }

    fn row_filter_hash(&mut self) -> &impl std::hash::Hash {
        &self.name_filter
    }

    fn filter_row(&mut self, row: &Row) -> bool {
        // filter by the first string column found, or "Name"
        let name_idx = self.column_configs.iter().position(|c| c.name.contains("Name"))
            .or_else(|| self.column_configs.iter().position(|c| c.column_type == ColumnType::String))
            .unwrap_or(0);

        if let Some(CellValue::String(name)) = row.cells.get(name_idx) {
            name.contains(&self.name_filter)
        } else {
            false
        }
    }

    fn show_cell_view(&mut self, ui: &mut egui::Ui, row: &Row, column: usize) {
        match &row.cells[column] {
            CellValue::String(s) => { ui.label(s); }
            CellValue::Int(i) => { ui.label(i.to_string()); }
            CellValue::Gender(g) => { ui.label(g.map(|gender: Gender|gender.to_string()).unwrap_or("Unspecified".to_string())); }
            CellValue::Bool(b) => { ui.checkbox(&mut { *b }, ""); }
            CellValue::Grade(g) => { ui.label(g.to_string()); }
        };
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
                        ColumnType::String => CellValue::String((*x).clone()),
                        ColumnType::Int => CellValue::Int(9999),
                        ColumnType::Gender => CellValue::Gender(Some(Gender::Female)),
                        ColumnType::Bool => CellValue::Bool(false),
                        ColumnType::Grade => CellValue::Grade(Grade::A),
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
        match &mut row.cells[column] {
            CellValue::String(s) => {
                egui::TextEdit::multiline(s)
                    .desired_rows(1)
                    .code_editor()
                    .show(ui)
                    .response
            }
            CellValue::Int(i) => ui.add(egui::DragValue::new(i).speed(1.0)),
            CellValue::Gender(gender) => {
                egui::ComboBox::new(ui.id().with("gender"), "".to_string())
                    .selected_text(gender.map(|gender: Gender|gender.to_string()).unwrap_or("Unspecified".to_string()))
                    .show_ui(ui, |ui|{
                        if ui
                            .add(egui::Button::selectable(
                                matches!(gender, Some(g) if *g == Gender::Male),
                                "Male"
                            ))
                            .clicked()
                        {
                            *gender = Some(Gender::Male);
                        }
                        if ui
                            .add(egui::Button::selectable(
                                matches!(gender, Some(g) if *g == Gender::Female),
                                "Female"
                            ))
                            .clicked()
                        {
                            *gender = Some(Gender::Female);
                        }

                    }).response
            }
            CellValue::Bool(b) => ui.checkbox(b, ""),
            CellValue::Grade(grade) => {
                ui.horizontal_wrapped(|ui| {
                    ui.radio_value(grade, Grade::A, "A")
                        | ui.radio_value(grade, Grade::B, "B")
                        | ui.radio_value(grade, Grade::C, "C")
                        | ui.radio_value(grade, Grade::D, "D")
                        | ui.radio_value(grade, Grade::E, "E")
                        | ui.radio_value(grade, Grade::F, "F")
                })
                    .inner
            }
        }
            .into()
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
            if let CellValue::Bool(is_student) = current.cells[idx] {
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
            if let CellValue::Bool(is_student) = row.cells[idx] {
                return !is_student;
            }
        }
        true
    }

    fn new_empty_row(&mut self) -> Row {
        let mut cells = Vec::with_capacity(self.column_configs.len());
        for config in &self.column_configs {
            let cell = match config.column_type {
                ColumnType::String => CellValue::String("".to_string()),
                ColumnType::Int => CellValue::Int(0),
                ColumnType::Gender => CellValue::Gender(None),
                ColumnType::Bool => CellValue::Bool(false),
                ColumnType::Grade => CellValue::Grade(Grade::F),
            };
            cells.push(cell);
        }
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
