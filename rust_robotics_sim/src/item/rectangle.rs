use super::{Point, Shape, Size, WithAngle, WithPosition, WithSize};
use egui::{plot::LineStyle, Color32, Stroke};

pub struct Rectangle {
    size: Size,
    angle: f64,
    position: Point,
    stroke: Stroke,
    fill_alpha: f32,
    style: LineStyle,
}

impl Default for Rectangle {
    fn default() -> Self {
        Self {
            position: Point::new(0.0, 0.0),
            size: Size::default(),
            angle: 0.0,
            stroke: Stroke::new(1.0, Color32::TRANSPARENT),
            style: LineStyle::Solid,
            fill_alpha: 0.05,
        }
    }
}

impl Rectangle {
    pub fn into_polygon(self) -> egui::plot::Polygon {
        egui::plot::Polygon::new(self.bounding_box())
            .fill_alpha(self.fill_alpha)
            .stroke(self.stroke)
            .style(self.style)
    }
}

crate::impl_size!(Rectangle);
crate::impl_angle!(Rectangle);
crate::impl_position!(Rectangle);
