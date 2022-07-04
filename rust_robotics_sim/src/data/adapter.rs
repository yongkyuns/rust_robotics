use super::*;

use egui::plot::{Value, Values};
use rust_robotics_algo as rb;

/// Allows converting from a column in [`TimeTable`] into [`egui`] [`Values`].
///
/// This is convenience trait for plotting on [`egui`] [`Plot`](egui::plot::Plot)
pub trait IntoValues {
    fn values(&self, column: usize) -> Option<Values>;
    fn values_shifted(&self, column: usize, x: f32, y: f32) -> Option<Values>;
}

impl IntoValues for TimeTable<f32> {
    fn values(&self, column: usize) -> Option<Values> {
        self.values_shifted(column, 0.0, 0.0)
    }
    fn values_shifted(&self, column: usize, x: f32, y: f32) -> Option<Values> {
        self.zipped_iter(column).map(|zip| {
            Values::from_values(
                zip.into_iter()
                    .map(|(t, v)| Value {
                        x: (*t + x) as f64,
                        y: (*v + y) as f64,
                    })
                    .collect(),
            )
        })
    }
}

/// Allows extracting information for [`plot`](egui::plot::Plot) from array of
/// 4-element vectors. The following assignments are assumed:
///
/// - [0] = x position
/// - [1] = y position
pub trait VehiclePlot {
    fn positions(&self) -> Values;
}

impl VehiclePlot for Vec<rb::Vector4> {
    fn positions(&self) -> Values {
        Values::from_values(
            self.into_iter()
                .map(|&state| Value {
                    x: *state.get(0).unwrap() as f64,
                    y: *state.get(1).unwrap() as f64,
                })
                .collect(),
        )
    }
}
