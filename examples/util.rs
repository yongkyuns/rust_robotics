#![allow(dead_code)]

use nannou::prelude::*;
use nannou::rand::{prelude::SliceRandom, thread_rng};

pub fn draw_dot(draw: &Draw, color: Srgb<u8>, x: f32, y: f32) {
    let r = 0.03;
    draw.ellipse().color(color).x_y(x, y).radius(r);
}

pub fn draw_line(draw: &Draw, color: Srgb<u8>, points: Vec<Point2>) {
    draw.polyline().weight(0.01).points(points).color(color);
}

pub fn draw_cart(draw: &Draw, x_pos: f32, angle: f32) {
    use rust_robotics_algo::inverted_pendulum::*;

    let model = Model::default();
    let l_bar = model.l_bar;

    let w = 1.0;
    let h = 0.5;
    let r = 0.1;
    let stroke = 0.01;
    let tol = 0.0001;

    // Draw cart
    draw.rect()
        .stroke(Color::navy().lighter())
        .stroke_weight(stroke)
        .color(Color::navy())
        .stroke_tolerance(tol)
        .join_round()
        .w_h(w, h)
        .x(x_pos);

    // Draw beam
    draw.line()
        .color(Color::orange())
        .stroke_weight(stroke)
        .start(Vec2::new(0.0, 0.0))
        .end(Vec2::new(0.0, l_bar))
        .x(x_pos)
        .y(h / 2.0)
        .z_radians(angle);

    // Draw right wheel
    draw.ellipse()
        .color(Color::coral())
        .stroke_color(Color::coral().lighter())
        .stroke_tolerance(tol)
        .stroke_weight(stroke)
        .x(x_pos + w / 4.0)
        .y(-h / 2.0 - r)
        .radius(r);

    // Draw right wheel
    draw.ellipse()
        .color(Color::coral())
        .stroke_color(Color::coral().lighter())
        .stroke_tolerance(tol)
        .stroke_weight(stroke)
        .x(x_pos - w / 4.0)
        .y(-h / 2.0 - r)
        .radius(r);

    // Draw ball on beam
    draw.ellipse()
        .color(Color::red())
        .stroke_color(Color::red().lighter())
        .stroke_tolerance(tol)
        .stroke_weight(stroke)
        .x(x_pos - l_bar * angle.sin())
        .y(h / 2.0 + l_bar * angle.cos())
        .radius(r);
}

pub fn draw_grid(draw: &Draw, win: &Rect, step: f32, weight: f32, zoom: f32, draw_text: bool) {
    let text_color = Rgb::<f32>::new(0.3, 0.3, 0.3);
    let font_size = 14;

    let step_by = || (0..).map(|i| i as f32 * step); // Grid spacing
    let r_iter = step_by().take_while(|&f| f < win.right());
    let l_iter = step_by().map(|f| -f).take_while(|&f| f > win.left());
    let x_iter = r_iter.chain(l_iter);
    for x in x_iter {
        draw.line()
            .weight(weight)
            .points(pt2(x, win.bottom()), pt2(x, win.top()));
        if draw_text {
            let text = format!("{:.1}", x);
            draw.text(&text)
                .x_y(x, 15.0)
                .color(text_color)
                .font_size(font_size);
        }
    }
    let t_iter = step_by().take_while(|&f| f < win.top());
    let b_iter = step_by().map(|f| -f).take_while(|&f| f > win.bottom());
    let y_iter = t_iter.chain(b_iter);
    for y in y_iter {
        draw.line()
            .weight(weight)
            .points(pt2(win.left(), y), pt2(win.right(), y));
    }
    draw.line()
        .weight(weight)
        .color(text_color)
        .points(pt2(0.0, win.bottom()), pt2(0.0, win.top()));
    draw.line()
        .weight(weight)
        .color(text_color)
        .points(pt2(win.left(), 0.0), pt2(win.right(), 0.0));

    let top = format!("{:.1}", win.top() / zoom);
    let bottom = format!("{:.1}", win.bottom() / zoom);
    let left = format!("{:.1}", win.left() / zoom);
    let right = format!("{:.1}", win.right() / zoom);
    let x_off = 30.0;
    let y_off = 20.0;
    draw.text("0.0")
        .x_y(15.0, 15.0)
        .color(text_color)
        .font_size(font_size);
    draw.text(&top)
        // .x_y(x_off, win.top() - 30.0)
        .x_y(x_off, 0.0)
        // .h(win.h())
        .font_size(font_size)
        .align_text_top()
        .color(text_color);
    // .x(x_off);
    draw.text(&bottom)
        .h(win.h())
        .font_size(font_size)
        .align_text_bottom()
        .color(text_color)
        .x(x_off);
    draw.text(&left)
        .w(win.w())
        .font_size(font_size)
        .left_justify()
        .color(text_color)
        .y(y_off);
    draw.text(&right)
        .w(win.w())
        .font_size(font_size)
        .right_justify()
        .color(text_color)
        .y(y_off);
}

pub struct Color {}

impl Color {
    pub fn navy() -> Rgb {
        rgb_from_hex(0x264653)
    }
    pub fn coral() -> Rgb {
        rgb_from_hex(0x2a9d8f)
    }
    pub fn yellow() -> Rgb {
        rgb_from_hex(0xe9c46a)
    }
    pub fn orange() -> Rgb {
        rgb_from_hex(0xf4a261)
    }
    pub fn red() -> Rgb {
        rgb_from_hex(0xe76f51)
    }
    pub fn palette() -> [Rgb; 5] {
        [
            Self::navy(),
            Self::coral(),
            Self::yellow(),
            Self::orange(),
            Self::red(),
        ]
    }
    pub fn random() -> Rgb {
        *Self::palette().choose(&mut thread_rng()).unwrap()
    }
}

pub trait Colored {
    fn lighter(&self) -> Rgb;
}

impl Colored for Rgb {
    fn lighter(&self) -> Rgb {
        let mut hsv: nannou::color::Hsv = self.into_linear().into();
        hsv.saturation -= 0.1;
        hsv.value += 0.2;
        hsv.into()
    }
}

pub fn rgb_from_hex(color: u32) -> Rgb {
    let color = nannou::color::rgb_u32(color);
    rgba(
        color.red as f32 / 255.0,
        color.green as f32 / 255.0,
        color.blue as f32 / 255.0,
        1.0,
    )
    .into()
}
