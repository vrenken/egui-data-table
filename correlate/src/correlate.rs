use std::{borrow::Cow, iter::repeat_with};
use egui::{Response, Sense, Widget};
use egui::scroll_area::ScrollBarVisibility;
use egui_data_table::{
    viewer::{default_hotkeys, CellWriteContext, DecodeErrorBehavior, RowCodec, UiActionContext},
    RowViewer,
};

use egui_material_icons::icons;

use crate::columns::*;
use crate::data::*;

/* ----------------------------------------- Data Scheme ---------------------------------------- */

struct Viewer {
    name_filter: String,
    row_protection: bool,
    hotkeys: Vec<(egui::KeyboardShortcut, egui_data_table::UiAction)>,
}



/* -------------------------------------------- Codec ------------------------------------------- */

struct Codec;

impl RowCodec<Row> for Codec {
    type DeserializeError = &'static str;

    fn create_empty_decoded_row(&mut self) -> Row {
        Row::default()
    }

    fn encode_column(&mut self, src_row: &Row, column: usize, dst: &mut String) {
        match column {
            NAME => dst.push_str(&src_row.name),
            AGE => dst.push_str(&src_row.age.to_string()),
            IS_STUDENT => dst.push_str(&src_row.is_student.to_string()),
            GRADE => dst.push_str(src_row.grade.to_string().as_str()),
            ROW_LOCKED => dst.push_str(&src_row.row_locked.to_string()),
            _ => unreachable!(),
        }
    }

    fn decode_column(
        &mut self,
        src_data: &str,
        column: usize,
        dst_row: &mut Row,
    ) -> Result<(), DecodeErrorBehavior> {
        match column {
            NAME => dst_row.name.replace_range(.., src_data),
            AGE => dst_row.age = src_data.parse().map_err(|_| DecodeErrorBehavior::SkipRow)?,
            IS_STUDENT => dst_row.is_student = src_data.parse().map_err(|_| DecodeErrorBehavior::SkipRow)?,
            GRADE => {
                dst_row.grade = src_data.parse().map_err(|_| DecodeErrorBehavior::SkipRow)?;
            }
            ROW_LOCKED => dst_row.row_locked = src_data.parse().map_err(|_| DecodeErrorBehavior::SkipRow)?,
            _ => unreachable!(),
        }

        Ok(())
    }
}

/* ------------------------------------ Viewer Implementation ----------------------------------- */

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
        Some(Codec)
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

/* ------------------------------------------ View Loop ----------------------------------------- */

pub struct CorrelateApp {
    table: egui_data_table::DataTable<Row>,
    viewer: Viewer,
    style_override: egui_data_table::Style,
    scroll_bar_always_visible: bool,
}

impl Default for CorrelateApp {

    // fn new(cc: &eframe::CreationContext<'_>) -> Self {
    //
    //     // register the fonts:
    //     egui_material_icons::initialize(&cc.egui_ctx);
    //
    //     Self::default()
    // }
    fn default() -> Self {
        Self {
            table: {
                let mut rng = fastrand::Rng::new();
                let mut name_gen = names::Generator::with_naming(names::Name::Numbered);

                repeat_with(move || {
                    Row {
                        name: name_gen.next().unwrap(),
                        age: rng.i32(4..31),
                        gender: match rng.i32(0..=2) {
                            0 => None,
                            1 => Some(Gender::Male),
                            2 => Some(Gender::Female),
                            _ => unreachable!(),
                        },
                        is_student: rng.bool(),
                        grade: rng.i32(0..=5).try_into().unwrap_or(Grade::F),
                        row_locked: false,
                    }
                })
            }
            .take(100000)
            .collect(),
            viewer: Viewer {
                name_filter: String::new(),
                hotkeys: Vec::new(),
                row_protection: false,
            },
            style_override: Default::default(),
            scroll_bar_always_visible: false,
        }
    }
}

impl eframe::App for CorrelateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        fn is_send<T: Send>(_: &T) {}
        fn is_sync<T: Sync>(_: &T) {}

        is_send(&self.table);
        is_sync(&self.table);

        egui::TopBottomPanel::top("MenuBar").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.hyperlink_to(
                    " kang-sw/egui-data-table",
                    "https://github.com/kang-sw/egui-data-table",
                );

                ui.hyperlink_to(
                    "(source)",
                    "https://github.com/kang-sw/egui-data-table/blob/master/examples/demo.rs",
                );

                ui.separator();

                egui::widgets::global_theme_preference_buttons(ui);

                ui.separator();

                ui.label("Name Filter");
                ui.text_edit_singleline(&mut self.viewer.name_filter);

                ui.add(egui::Button::new("Drag me and drop on any cell").sense(Sense::drag()))
                    .on_hover_text(
                        "Dropping this will replace the cell \
                        content with some predefined value.",
                    )
                    .dnd_set_drag_payload(String::from("Hallo~"));

                ui.menu_button("🎌 Flags", |ui| {
                    ui.checkbox(&mut self.viewer.row_protection, "Row Protection")
                        .on_hover_text(
                            "If checked, any rows `Is Student` marked \
                        won't be deleted or overwritten by UI actions.",
                        );

                    ui.checkbox(
                        &mut self.style_override.single_click_edit_mode,
                        "Single Click Edit",
                    )
                    .on_hover_text("If checked, cells will be edited with a single click.");

                    ui.checkbox(
                        &mut self.style_override.auto_shrink.x,
                        "Auto-shrink X",
                    );
                    ui.checkbox(
                        &mut self.style_override.auto_shrink.y,
                        "Auto-shrink Y",
                    );

                    ui.checkbox(
                        &mut self.scroll_bar_always_visible,
                        "Scrollbar always visible",
                    );

                    if ui.button("Shuffle Rows").clicked() {
                        fastrand::shuffle(&mut self.table);
                    }
                })
            })
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            egui::Sides::new().show(ui, |_ui| {
            }, |ui|{
                let mut has_modifications = self.table.has_user_modification();
                ui.add_enabled(false, egui::Checkbox::new(&mut has_modifications, "Has modifications"));

                ui.add_enabled_ui(has_modifications, |ui| {
                    if ui.button("Clear").clicked() {
                        self.table.clear_user_modification_flag();
                    }
                });
            });
        });
        
        egui::SidePanel::left("Hotkeys")
            .default_width(500.)
            .show(ctx, |ui| {
                ui.vertical_centered_justified(|ui| {
                    ui.heading("Hotkeys");
                    ui.separator();
                    ui.add_space(0.);

                    for (k, a) in &self.viewer.hotkeys {
                        egui::Button::new(format!("{a:?}"))
                            .shortcut_text(ctx.format_shortcut(k))
                            .wrap_mode(egui::TextWrapMode::Wrap)
                            .sense(Sense::hover())
                            .ui(ui);
                    }
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.scroll_bar_always_visible {
                true => {
                    ui.style_mut().spacing.scroll = egui::style::ScrollStyle::solid();
                    self.style_override.scroll_bar_visibility = ScrollBarVisibility::AlwaysVisible;
                },
                false => {
                    ui.style_mut().spacing.scroll = egui::style::ScrollStyle::floating();
                    self.style_override.scroll_bar_visibility = ScrollBarVisibility::VisibleWhenNeeded;
                }
            };

            ui.add(
                egui_data_table::Renderer::new(&mut self.table, &mut self.viewer)
                    .with_style(self.style_override),
            )
        });
    }
}
