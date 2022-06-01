#![allow(non_snake_case, non_upper_case_globals)]

use nannou::prelude::*;
use rust_robotics_algo as rb;
use rust_robotics_algo::pf::*;
use rust_robotics_algo::prelude::*;

mod util;
use util::*;

const rf_id: [rb::Vector2; 4] = [
    vector![10.0_f32, 0.0_f32],
    vector![10.0, 10.0],
    vector![0.0, 15.0],
    vector![-5.0, 20.0],
];

const N: usize = 1000;

struct Pan {
    start: Point2,
    end: Point2,
    active: bool,
}

impl Pan {
    pub fn new() -> Self {
        Self {
            start: pt2(0.0, 0.0),
            end: pt2(0.0, 0.0),
            active: false,
        }
    }
    pub fn now(&self) -> Point2 {
        self.end - self.start
    }
    pub fn is_active(&self) -> bool {
        self.active
    }
    pub fn start(&mut self, cursor: Point2) {
        self.start = cursor;
        self.end = cursor;
        self.active = true;
    }
    pub fn end(&mut self) {
        self.start = pt2(0.0, 0.0);
        self.active = false;
        self.end = pt2(0.0, 0.0);
    }
    pub fn update(&mut self, cursor: Point2) {
        self.end = cursor;
    }
}

struct ViewState {
    origin: Point2,
    zoom: f32,
    pan: Pan,
}

impl ViewState {
    pub fn new() -> Self {
        Self {
            origin: pt2(0.0, 0.0),
            zoom: 1.0,
            pan: Pan::new(),
        }
    }
    pub fn pan(&self) -> Point2 {
        self.origin + self.pan.now()
    }
    pub fn start_pan(&mut self, cursor: Point2) {
        self.pan.start(cursor);
    }
    pub fn end_pan(&mut self) {
        self.origin += self.pan.now();
        self.pan.end();
    }
}

struct ParticleFilter {
    x_est: rb::Vector4,
    x_true: rb::Vector4,
    x_dr: rb::Vector4,
    p_est: rb::Matrix3,
    pw: PW,
    px: PX,
    h_x_est: Vec<rb::Vector4>,
    h_x_true: Vec<rb::Vector4>,
    h_x_dr: Vec<rb::Vector4>,
    view: ViewState,
}

impl ParticleFilter {
    pub fn new() -> Self {
        Self {
            x_est: zeros!(4, 1),
            x_true: zeros!(4, 1),
            x_dr: zeros!(4, 1),
            p_est: zeros!(3, 3),
            pw: ones!(1, NP) * (1. / NP as f32),
            px: zeros!(4, NP),
            h_x_est: vec![zeros!(4, 1)],
            h_x_true: vec![zeros!(4, 1)],
            h_x_dr: vec![zeros!(4, 1)],
            view: ViewState::new(),
        }
    }

    pub fn update_history(&mut self) {
        self.h_x_est.push(self.x_est);
        self.h_x_true.push(self.x_true);
        self.h_x_dr.push(self.x_dr);

        if self.h_x_est.len() > N {
            self.h_x_est.remove(0);
            self.h_x_true.remove(0);
            self.h_x_dr.remove(0);
        }
    }

    pub fn step(&mut self, dt: f32) {
        let u = calc_input();
        let (z, ud) = observation(&mut self.x_true, &mut self.x_dr, u, &rf_id, dt);
        self.p_est = pf_localization(&mut self.x_est, &mut self.px, &mut self.pw, z, ud, dt);

        self.update_history();

        // // println!("t = {}, Input :{}", now.elapsed().as_secs_f32(), u);
        // // println!("{}", u);
    }
}

fn get_hist_pos(hist: &[rb::Vector4]) -> Vec<Point2> {
    hist.iter().map(|state| pt2(state[0], state[1])).collect()
}

fn main() {
    nannou::app(model).update(update).view(draw).run()
}

fn model(app: &App) -> ParticleFilter {
    app.new_window()
        .size(800, 600)
        .key_pressed(key_pressed)
        .mouse_wheel(mouse_wheel)
        .mouse_moved(mouse_moved)
        .mouse_pressed(mouse_pressed)
        .mouse_released(mouse_released)
        .mouse_exited(mouse_exited)
        .build()
        .unwrap();
    ParticleFilter::new()
}

fn update(app: &App, pf: &mut ParticleFilter, _update: Update) {
    let dt = 1.0 / app.fps();
    pf.step(dt);
}

fn draw(app: &App, pf: &ParticleFilter, frame: Frame) {
    let draw = app.draw().translate((pf.view.pan(), 0.0).into());
    draw.background().rgb(0.11, 0.12, 0.13);
    let draw = draw.scale(pf.view.zoom);
    let mut bounds = app.main_window().rect().shift(-pf.view.pan());
    // let zoom = (pf.view.zoom * 100.0).max(1e-3);
    let zoom = pf.view.zoom.max(1e-3);
    bounds.x.start /= zoom;
    bounds.x.end /= zoom;
    bounds.y.start /= zoom;
    bounds.y.end /= zoom;

    let major_tick = 100.0;
    let minor_tick = 25.0;
    draw_grid(&draw, &bounds, major_tick, 1.0, zoom, true);
    draw_grid(&draw, &bounds, minor_tick, 0.5, zoom, false);

    draw_line(&draw, GREEN, get_hist_pos(&pf.h_x_dr));
    draw_line(&draw, BLUE, get_hist_pos(&pf.h_x_true));
    draw_line(&draw, RED, get_hist_pos(&pf.h_x_est));

    draw_dot(&draw, GREEN, pf.x_dr.x, pf.x_dr.y);
    draw_dot(&draw, BLUE, pf.x_true.x, pf.x_true.y);
    draw_dot(&draw, RED, pf.x_est.x, pf.x_est.y);

    draw.to_frame(app, &frame).unwrap();
}

fn key_pressed(_app: &App, pf: &mut ParticleFilter, key: Key) {
    if key == Key::Return || key == Key::Space {
        *pf = ParticleFilter::new();
    }
}

fn mouse_wheel(_app: &App, pf: &mut ParticleFilter, delta: MouseScrollDelta, _touch: TouchPhase) {
    match delta {
        MouseScrollDelta::PixelDelta(pos) => pf.view.zoom += 0.01 * pos.y as f32,
        _ => (),
    }
}

fn mouse_moved(_app: &App, pf: &mut ParticleFilter, pos: Point2) {
    if pf.view.pan.is_active() {
        pf.view.pan.update(pos);
    }
}

fn mouse_pressed(app: &App, pf: &mut ParticleFilter, button: MouseButton) {
    match button {
        MouseButton::Left => {
            pf.view.start_pan(app.mouse.position());
        }
        _ => (),
    }
}

fn mouse_released(_app: &App, pf: &mut ParticleFilter, button: MouseButton) {
    match button {
        MouseButton::Left => {
            pf.view.end_pan();
        }
        _ => (),
    }
}

fn mouse_exited(_app: &App, pf: &mut ParticleFilter) {
    pf.view.end_pan();
}
