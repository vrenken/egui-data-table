use egui::{Response, Ui};
use crate::data::*;

pub trait ColumnTypeEditor {
    fn show(
        &self,
        ui: &mut Ui,
        cell_value: &mut CellValue,
        column_config: &mut ColumnConfig,
    ) -> Option<Response>;
}

pub fn get_random_gentle_color() -> [u8; 3] {
    let h = fastrand::f32();
    let s = 0.5; // gentle saturation
    let l = 0.8; // gentle lightness
    
    let color = egui::ecolor::Hsva::new(h, s, l, 1.0);
    let rgb = egui::Color32::from(color);
    [rgb.r(), rgb.g(), rgb.b()]
}

pub mod text;
pub use text::*;

pub mod number;
pub use number::*;

pub mod date_time;
pub use date_time::*;
pub mod bool;
pub use bool::*;

pub mod select;
pub use select::*;

pub mod multi_select;
pub use multi_select::*;

pub mod relation;
pub use relation::*;
