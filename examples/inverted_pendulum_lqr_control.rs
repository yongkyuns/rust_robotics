#![allow(non_snake_case)]

use nannou::prelude::*;
use rust_robotics_algo as rb;
use rust_robotics_algo::lqr::*;
use rust_robotics_algo::prelude::*;

mod util;
use util::*;

struct InvertedPendulum {
    state: rb::Vector4,
}

impl InvertedPendulum {
    pub fn new() -> Self {
        Self {
            state: vector![0., 0., random_range(-0.4, 0.4), 0.],
        }
    }

    pub fn step(&mut self, dt: f32) {
        let mut x = self.state.clone();
        let (A, B) = get_model_matrix(dt);

        // let now = Instant::now();

        // Perform LQR control
        let u = lqr_control(x, dt);

        // Update simulation based on control input
        x = A * x + B * u;
        self.state = x;

        // println!("t = {}, Input :{}", now.elapsed().as_secs_f32(), u);
        // println!("{}", u);
    }
}

fn main() {
    nannou::app(model).update(update).view(draw).run()
}

fn model(app: &App) -> InvertedPendulum {
    app.new_window()
        .size(800, 600)
        .key_pressed(key_pressed)
        .build()
        .unwrap();
    InvertedPendulum::new()
}

fn update(app: &App, pendulum: &mut InvertedPendulum, _update: Update) {
    let dt = 1.0 / app.fps();
    pendulum.step(dt);
}

fn draw(app: &App, pendulum: &InvertedPendulum, frame: Frame) {
    let draw = app.draw();
    let win_rect = app.main_window().rect();
    draw.background().rgb(0.11, 0.12, 0.13);

    let x_pos = pendulum.state[0];
    let angle = pendulum.state[2];

    let zoom = 100.0;

    draw_grid(&draw, &win_rect, 100.0, 1.0, zoom, true);
    draw_grid(&draw, &win_rect, 25.0, 0.5, zoom, false);

    let draw = draw.scale(zoom);

    draw_cart(&draw, x_pos, angle);

    draw.to_frame(app, &frame).unwrap();
}

fn key_pressed(_app: &App, pendulum: &mut InvertedPendulum, key: Key) {
    if key == Key::Return || key == Key::Space {
        *pendulum = InvertedPendulum::new();
    }
}
