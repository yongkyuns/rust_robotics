#![allow(non_snake_case)]

use egui::plot::PlotUi;
use rand::Rng;
use rust_robotics_algo as rb;
use rust_robotics_algo::lqr::*;
use rust_robotics_algo::prelude::*;

use crate::prelude::draw_cart;

use super::{SimType, Simulate};

pub struct InvertedPendulum {
    state: rb::Vector4,
}

impl Default for InvertedPendulum {
    fn default() -> Self {
        Self {
            state: vector![0., 0., rand::thread_rng().gen_range(-0.4..0.4), 0.],
        }
    }
}

impl InvertedPendulum {
    pub fn _new() -> Self {
        Self::default()
    }

    pub fn x_position(&self) -> f32 {
        self.state[0]
    }

    pub fn rod_angle(&self) -> f32 {
        self.state[2]
    }
}

impl Simulate for InvertedPendulum {
    fn sim_type(&self) -> SimType {
        SimType::InvertedPendulum
    }
    fn match_states(&mut self, other: Box<dyn Simulate>) {}
    fn step(&mut self, dt: f32) {
        let mut x = self.state.clone();
        let (A, B) = get_model_matrix(dt);

        // Perform LQR control
        let u = lqr_control(x, dt);

        // Update simulation based on control input
        x = A * x + B * u;
        self.state = x;
    }
    fn reset(&mut self) {
        *self = Self::default();
    }
    fn draw(&self, plot_ui: &mut PlotUi) {
        draw_cart(plot_ui, self.x_position(), self.rod_angle());
    }
}
