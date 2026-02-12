use std::borrow::Cow;
use egui::Response;
use egui_data_table::RowViewer;
use egui_data_table::viewer::{default_hotkeys, CellWriteContext, RowCodec, UiActionContext};
use crate::columns::{AGE, COLUMN_COUNT, COLUMN_NAMES, GENDER, GRADE, IS_STUDENT, NAME, ROW_LOCKED};
use crate::data::{Gender, Grade, Row};

pub struct Viewer {
    pub name_filter: String,
    pub row_protection: bool,
    pub hotkeys: Vec<(egui::KeyboardShortcut, egui_data_table::UiAction)>,
}

impl RowViewer<Row> for Viewer {
    fn num_columns(&mut self) -> usize {
        COLUMN_COUNT
    }

    fn column_name(&mut self, column: usize) -> Cow<'static, str> {
        COLUMN_NAMES.get(column)
            .copied()
            .map(Cow::Borrowed)
            .unwrap_or_else(|| Cow::Owned(format!("Column {}", column)))
    }

    fn try_create_codec(&mut self, _: bool) -> Option<impl RowCodec<Row>> {
        Some(crate::correlate::Codec)
    }

    fn is_sortable_column(&mut self, column: usize) -> bool {
        [true, true, true, false, true, true][column]
    }

    fn is_editable_cell(&mut self, column: usize, _row: usize, row_value: &Row) -> bool {
        let row_locked = row_value.row_locked;
        // allow editing of the locked flag, but prevent editing other columns when locked.
        match column {
            ROW_LOCKED => true,
            _ => !row_locked,
        }
    }

    fn compare_cell(&self, row_l: &Row, row_r: &Row, column: usize) -> std::cmp::Ordering {
        match column {
            NAME => row_l.name.cmp(&row_r.name),
            AGE => row_l.age.cmp(&row_r.age),
            GENDER => row_l.gender.cmp(&row_r.gender),
            IS_STUDENT => unreachable!(),
            GRADE => row_l.grade.cmp(&row_r.grade),
            ROW_LOCKED => row_l.row_locked.cmp(&row_r.row_locked),
            _ => unreachable!(),
        }
    }

    fn row_filter_hash(&mut self) -> &impl std::hash::Hash {
        &self.name_filter
    }

    fn filter_row(&mut self, row: &Row) -> bool {
        row.name.contains(&self.name_filter)
    }

    fn show_cell_view(&mut self, ui: &mut egui::Ui, row: &Row, column: usize) {
        let _ = match column {
            NAME => ui.label(&row.name),
            AGE => ui.label(row.age.to_string()),
            GENDER => ui.label(row.gender.map(|gender: Gender|gender.to_string()).unwrap_or("Unspecified".to_string())),
            IS_STUDENT => ui.checkbox(&mut { row.is_student }, ""),
            GRADE => ui.label(row.grade.to_string()),
            ROW_LOCKED => ui.checkbox(&mut { row.row_locked }, ""),

            _ => unreachable!(),
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
                Box::new(Row{
                    name: (*x).clone(),
                    age: 9999,
                    gender: Some(Gender::Female),
                    is_student: false,
                    grade: Grade::A,
                    row_locked: false
                })
            })
    }

    fn show_cell_editor(
        &mut self,
        ui: &mut egui::Ui,
        row: &mut Row,
        column: usize,
    ) -> Option<Response> {
        match column {
            NAME => {
                egui::TextEdit::multiline(&mut row.name)
                    .desired_rows(1)
                    .code_editor()
                    .show(ui)
                    .response
            }
            AGE => ui.add(egui::DragValue::new(&mut row.age).speed(1.0)),
            GENDER => {
                let gender = &mut row.gender;

                egui::ComboBox::new(ui.id().with("gender"), "".to_string())
                    .selected_text(gender.map(|gender: Gender|gender.to_string()).unwrap_or("Unspecified".to_string()))
                    .show_ui(ui, |ui|{
                        if ui
                            .add(egui::Button::selectable(
                                matches!(gender, Some(gender) if *gender == Gender::Male),
                                "Male"
                            ))
                            .clicked()
                        {
                            *gender = Some(Gender::Male);
                        }
                        if ui
                            .add(egui::Button::selectable(
                                matches!(gender, Some(gender) if *gender == Gender::Female),
                                "Female"
                            ))
                            .clicked()
                        {
                            *gender = Some(Gender::Female);
                        }

                    }).response
            }
            IS_STUDENT => ui.checkbox(&mut row.is_student, ""),
            GRADE => {
                let grade = &mut row.grade;
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
            ROW_LOCKED => ui.checkbox(&mut row.row_locked, ""),
            _ => unreachable!(),
        }
            .into()
    }

    fn set_cell_value(&mut self, src: &Row, dst: &mut Row, column: usize) {
        match column {
            NAME => dst.name.clone_from(&src.name),
            AGE => dst.age = src.age,
            GENDER => dst.gender = src.gender,
            IS_STUDENT => dst.is_student = src.is_student,
            GRADE => dst.grade = src.grade,
            ROW_LOCKED => dst.row_locked = src.row_locked,
            _ => unreachable!(),
        }
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

        !current.is_student
    }

    fn confirm_row_deletion_by_ui(&mut self, row: &Row) -> bool {
        if !self.row_protection {
            return true;
        }

        !row.is_student
    }

    fn new_empty_row(&mut self) -> Row {
        Row::default()
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
