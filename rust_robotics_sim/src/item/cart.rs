use super::{Circle, Point, Rectangle, Shape, WithPosition, WithSize};
use crate::math::{cos, sin};
use egui::plot::{Line, PlotUi, Values};

// pub struct Cart {
//     size: Size,
//     angle: f64,
//     position: Point,
// }

// impl Default for Cart {
//     fn default() -> Self {
//         Self {
//             position: Point::new(0.0, 0.0),
//             size: Size::default(),
//             angle: 0.0,
//         }
//     }
// }

// impl Cart {
//     pub fn plot(&mut self, plot_ui: &mut PlotUi, x_pos: f64, rod_angle: f64) {
//         let x = x_pos;
//         let y = self.position().y;

//         // let x = self.position().x;
//         // let y = self.position().y;

//         let r = 0.1;
//         let w = 1.0;
//         let h = 0.5;
//         let len = 2.0;
//         let th = rod_angle;
//         // let th = self.angle();

//         let rod_bottom = Point::new(x, y + h + 2.0 * r);
//         let rod_top = Point::new(rod_bottom.x - len * sin(th), rod_bottom.y + len * cos(th));

//         let body = Rectangle::new()
//             .with_width(w)
//             .with_height(h)
//             .at(x, y + h / 2.0 + 2.0 * r)
//             .into_polygon();
//         let left_wheel = Circle::new()
//             .with_radius(r)
//             .at(x - w / 4.0, y + r)
//             .into_polygon();
//         let right_wheel = Circle::new()
//             .with_radius(r)
//             .at(x + w / 4.0, y + r)
//             .into_polygon();
//         let ball = Circle::new()
//             .with_radius(r)
//             .at(rod_top.x, rod_top.y)
//             .into_polygon();
//         let rod = Line::new(Values::from_values(vec![rod_bottom, rod_top]));

//         plot_ui.polygon(body.name("Cart"));
//         plot_ui.polygon(left_wheel);
//         plot_ui.polygon(right_wheel);
//         plot_ui.polygon(ball);
//         plot_ui.line(rod);
//     }
// }

// crate::impl_size!(Cart);
// crate::impl_angle!(Cart);
// crate::impl_position!(Cart);

pub fn draw_cart(plot_ui: &mut PlotUi, x_pos: f32, rod_angle: f32) {
    let x = x_pos as f64;
    let y = 0.0;

    // let x = self.position().x;
    // let y = self.position().y;

    let r = 0.1;
    let w = 1.0;
    let h = 0.5;
    let len = 2.0;
    let th = rod_angle as f64;

    let rod_bottom = Point::new(x, y + h + 2.0 * r);
    let rod_top = Point::new(rod_bottom.x - len * sin(th), rod_bottom.y + len * cos(th));

    let body = Rectangle::new()
        .with_width(w)
        .with_height(h)
        .at(x, y + h / 2.0 + 2.0 * r)
        .into_polygon();
    let left_wheel = Circle::new()
        .with_radius(r)
        .at(x - w / 4.0, y + r)
        .into_polygon();
    let right_wheel = Circle::new()
        .with_radius(r)
        .at(x + w / 4.0, y + r)
        .into_polygon();
    let ball = Circle::new()
        .with_radius(r)
        .at(rod_top.x, rod_top.y)
        .into_polygon();
    let rod = Line::new(Values::from_values(vec![rod_bottom, rod_top]));

    plot_ui.polygon(body.name("Cart"));
    plot_ui.polygon(left_wheel);
    plot_ui.polygon(right_wheel);
    plot_ui.polygon(ball);
    plot_ui.line(rod);
}
