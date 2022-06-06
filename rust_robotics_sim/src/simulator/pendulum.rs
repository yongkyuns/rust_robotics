#![allow(non_snake_case)]

use super::Draw;
use crate::prelude::draw_cart;

use egui::{plot::PlotUi, Ui};
use rand::Rng;
use rust_robotics_algo as rb;
use rust_robotics_algo::inverted_pendulum::*;
use rust_robotics_algo::prelude::*;

use super::{SimType, Simulate};

/// Controller for the inverted pendulum simulation
pub enum Controller {
    LQR,
}

impl Default for Controller {
    fn default() -> Self {
        Self::LQR
    }
}

/// Inverted pendulum simulation
pub struct InvertedPendulum {
    state: rb::Vector4,
    controller: Controller,
    model: Model,
}

impl Default for InvertedPendulum {
    fn default() -> Self {
        Self {
            state: vector![0., 0., rand::thread_rng().gen_range(-0.4..0.4), 0.],
            controller: Controller::default(),
            model: Model::default(),
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
        let (A, B) = self.model.get_model_matrix(dt);

        // Perform LQR control
        let u = lqr_control(x, &self.model, dt);

        // Update simulation based on control input
        x = A * x + B * u;
        self.state = x;
    }
    fn reset(&mut self) {
        *self = Self::default();
    }
}

impl Draw for InvertedPendulum {
    fn draw(&self, plot_ui: &mut PlotUi) {
        draw_cart(plot_ui, self.x_position(), self.rod_angle(), &self.model);
    }
    fn options_ui(&mut self, ui: &mut Ui) {
        // let Self {
        //     animate,
        //     time: _,
        //     circle_radius,
        //     circle_center,
        //     square,
        //     proportional,
        //     line_style,
        //     coordinates,
        //     ..
        // } = self;

        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.label("Cart:");
                    ui.add(
                        egui::DragValue::new(&mut self.model.l_bar)
                            .speed(0.01)
                            .clamp_range(0.1_f32..=10.0)
                            .prefix("Beam Length: "),
                    );
                    ui.horizontal(|ui| {
                        ui.add(
                            egui::DragValue::new(&mut self.model.M)
                                .speed(0.01)
                                .clamp_range(0.1_f32..=3.0)
                                .prefix("Cart Mass: "),
                        );
                        ui.add(
                            egui::DragValue::new(&mut self.model.m)
                                .speed(0.01)
                                .clamp_range(0.1_f32..=10.0)
                                .prefix("Ball Mass: "),
                        );
                    });
                    ui.horizontal(|ui| {
                        if ui.button("Restart").clicked() {
                            self.state =
                                vector![0., 0., rand::thread_rng().gen_range(-0.4..0.4), 0.];
                        }
                        if ui.button("Reset All").clicked() {
                            self.reset();
                        }
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
