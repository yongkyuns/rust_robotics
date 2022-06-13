#![allow(non_snake_case)]

use super::Draw;
use crate::prelude::draw_cart;

use egui::{plot::PlotUi, ComboBox, Ui};
use rand::Rng;
use rb::inverted_pendulum::*;
use rb::prelude::*;
use rust_robotics_algo as rb;

use super::Simulate;

/// Controller for the inverted pendulum simulation
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Controller {
    LQR(Model),
    PID(PID),
}

// impl Default for Controller {
//     fn default() -> Self {
//         Self::LQR
//     }
// }

impl Controller {
    pub fn control(&mut self, x: rb::Vector4, dt: f32) -> f32 {
        match self {
            Self::LQR(model) => lqr_control(x, model, dt),
            Self::PID(pid) => pid.control(-x[2], dt),
        }
    }
    /// Instantiate a new LQR controller for [`InvertedPendulum`]
    pub fn lqr(model: Model) -> Self {
        Self::LQR(model)
    }
    /// Instantiate a new PID controller for [`InvertedPendulum`]
    pub fn pid() -> Self {
        Self::PID(PID::default())
    }
    /// Reset the states of the current [`Controller`]
    ///
    /// If there are parameters related to the controller (e.g. PID gains),
    /// this method retains those parameters unchanged and only resets the
    /// internal states (e.g. integral error in [`PID controller`](PID))
    pub fn reset_state(&mut self) {
        match self {
            Self::LQR(_) => (),
            Self::PID(pid) => pid.reset_state(),
        }
    }
    /// Reset the states and any parameters to it's default values
    ///
    /// This method only retains the [`Controller`] selection but resets
    /// any internal states and parameters. If you want to only reset the
    /// the state of a controller (e.g. integral error of PID control), use
    /// [`reset_state`](Controller::reset_state) instead.
    pub fn reset_all(&mut self) {
        match self {
            Self::LQR(_) => *self = Self::lqr(Model::default()),
            Self::PID(_) => *self = Self::pid(),
        }
    }
    /// Method to draw onto [`egui`] UI.
    pub fn options_ui(&mut self, ui: &mut Ui) {
        match self {
            Self::LQR(model) => {
                ui.vertical(|ui| {
                    ui.label("LQR Model Parameters:");
                    ui.add(
                        egui::DragValue::new(&mut model.l_bar)
                            .speed(0.01)
                            .clamp_range(0.1_f32..=10.0)
                            .prefix("Beam Length: ")
                            .suffix(" m"),
                    );
                    ui.add(
                        egui::DragValue::new(&mut model.M)
                            .speed(0.01)
                            .clamp_range(0.1_f32..=3.0)
                            .prefix("Cart Mass: ")
                            .suffix(" kg"),
                    );
                    ui.add(
                        egui::DragValue::new(&mut model.m)
                            .speed(0.01)
                            .clamp_range(0.1_f32..=10.0)
                            .prefix("Ball Mass: ")
                            .suffix(" kg"),
                    );
                });
            }
            Self::PID(pid) => {
                ui.vertical(|ui| {
                    ui.label("LQR Model Parameters:");
                    ui.add(
                        egui::DragValue::new(&mut pid.P)
                            .speed(0.01)
                            .clamp_range(0.01_f32..=10000.0)
                            .prefix("P gain"),
                    );
                    ui.add(
                        egui::DragValue::new(&mut pid.I)
                            .speed(0.01)
                            .clamp_range(0.01_f32..=10000.0)
                            .prefix("I gain"),
                    );
                    ui.add(
                        egui::DragValue::new(&mut pid.D)
                            .speed(0.01)
                            .clamp_range(0.01_f32..=10000.0)
                            .prefix("D gain"),
                    );
                });
            }
        }
    }

    /// Output the [`String`] for the currrent controller
    pub fn to_string(&self) -> String {
        match self {
            Self::LQR(_) => "LQR".to_owned(),
            Self::PID(_) => "PID".to_owned(),
        }
    }
}

/// Inverted pendulum simulation
pub struct InvertedPendulum {
    state: rb::Vector4,
    controller: Controller,
    model: Model,
    id: usize,
}

impl Default for InvertedPendulum {
    fn default() -> Self {
        Self {
            state: vector![0., 0., rand::thread_rng().gen_range(-0.4..0.4), 0.],
            controller: Controller::lqr(Model::default()),
            model: Model::default(),
            id: 1,
        }
    }
}

impl InvertedPendulum {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    pub fn x_position(&self) -> f32 {
        self.state[0]
    }

    pub fn rod_angle(&self) -> f32 {
        self.state[2]
    }
}

impl Simulate for InvertedPendulum {
    fn get_state(&self) -> &dyn std::any::Any {
        &self.state
    }
    fn match_state_with(&mut self, other: &dyn Simulate) {
        if let Some(data) = other.get_state().downcast_ref::<rb::Vector4>() {
            // Then set self's data from `other` if the type matches
            self.state.clone_from(data);
        }
    }
    fn step(&mut self, dt: f32) {
        let mut x = self.state.clone();
        let (A, B) = self.model.get_model_matrix(dt);

        // Compute control command
        let u = self.controller.control(x, dt);

        // Update simulation based on control input
        x = A * x + B * u;
        self.state = x;
    }
    fn reset_state(&mut self) {
        self.state = vector![0., 0., rand::thread_rng().gen_range(-0.4..0.4), 0.];
        self.controller.reset_state();
    }
    fn reset_all(&mut self) {
        *self = Self::default();
    }
}

impl Draw for InvertedPendulum {
    fn draw(&self, plot_ui: &mut PlotUi) {
        draw_cart(plot_ui, self.x_position(), self.rod_angle(), &self.model);
    }
    fn options_ui(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label("Cart:");
                        ui.add(
                            egui::DragValue::new(&mut self.model.l_bar)
                                .speed(0.01)
                                .clamp_range(0.1_f32..=10.0)
                                .prefix("Beam Length: ")
                                .suffix(" m"),
                        );
                        ui.add(
                            egui::DragValue::new(&mut self.model.M)
                                .speed(0.01)
                                .clamp_range(0.1_f32..=3.0)
                                .prefix("Cart Mass: ")
                                .suffix(" kg"),
                        );
                        ui.add(
                            egui::DragValue::new(&mut self.model.m)
                                .speed(0.01)
                                .clamp_range(0.1_f32..=10.0)
                                .prefix("Ball Mass: ")
                                .suffix(" kg"),
                        );
                        ui.group(|ui| {
                            ui.vertical(|ui| {
                                ui.push_id(self.id, |ui| {
                                    ComboBox::from_label("Controller")
                                        .selected_text(self.controller.to_string())
                                        .show_ui(ui, |ui| {
                                            for options in
                                                [Controller::lqr(self.model), Controller::pid()]
                                                    .iter()
                                            {
                                                ui.selectable_value(
                                                    &mut self.controller,
                                                    *options,
                                                    options.to_string(),
                                                );
                                            }
                                        });
                                });
                                self.controller.options_ui(ui);
                            });
                        });
                    });
                });
            });

            // ui.vertical(|ui| {
            //     ui.style_mut().wrap = Some(false);
            //     ui.checkbox(animate, "Animate");
            //     ui.checkbox(square, "Square view")
            //         .on_hover_text("Always keep the viewport square.");
            //     ui.checkbox(proportional, "Proportional data axes")
            //         .on_hover_text("Tick are the same size on both axes.");
            //     ui.checkbox(coordinates, "Show coordinates")
            //         .on_hover_text("Can take a custom formatting function.");

            //     ComboBox::from_label("Line style")
            //         .selected_text(line_style.to_string())
            //         .show_ui(ui, |ui| {
            //             for style in [
            //                 LineStyle::Solid,
            //                 LineStyle::dashed_dense(),
            //                 LineStyle::dashed_loose(),
            //                 LineStyle::dotted_dense(),
            //                 LineStyle::dotted_loose(),
            //             ]
            //             .iter()
            //             {
            //                 ui.selectable_value(line_style, *style, style.to_string());
            //             }
            //         });
            // });
        });
    }
}
