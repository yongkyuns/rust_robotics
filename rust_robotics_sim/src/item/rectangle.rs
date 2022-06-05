use super::{Point, Shape, Size, WithAngle, WithPosition, WithSize};

pub struct Rectangle {
    size: Size,
    angle: f64,
    position: Point,
}

impl Default for Rectangle {
    fn default() -> Self {
        Self {
            position: Point::new(0.0, 0.0),
            size: Size::default(),
            angle: 0.0,
        }
    }
}

impl Rectangle {
    pub fn into_polygon(self) -> egui::plot::Polygon {
        egui::plot::Polygon::new(self.bounding_box())
    }
}

crate::impl_size!(Rectangle);
crate::impl_angle!(Rectangle);
crate::impl_position!(Rectangle);
