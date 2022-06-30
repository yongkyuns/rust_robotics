pub mod pendulum;

use crate::prelude::*;
use pendulum::InvertedPendulum;

use egui::{
    plot::{Line, PlotUi, Values},
    *,
};
use plot::{Corner, Legend, Plot};

/// Base trait to make simulation work within `rust robotics`.
///
/// Users can implement this trait to make custom simulations.
pub trait Simulate {
    /// Getter method for internal state of an object that implements [`Simulate`]
    ///
    /// This method allows simulations of same type to communicate its internal
    /// state. The usecase for this method is when we want to align the initial
    /// conditions of multiple simulations, so that they can be compared with
    /// respect to each other throughout the simulation.
    fn get_state(&self) -> &dyn std::any::Any;

    /// Match the current simulation's state with that of another object, as long
    /// as it's state is compatible with the current simulation.
    fn match_state_with(&mut self, other: &dyn Simulate);

    /// Take a single step through simulation based on the given time delta
    fn step(&mut self, dt: f32);

    /// Reset the dynamic states of the current simulation object.
    ///
    /// Any dynamic states that get updated with [`Simulate::step`] should be
    /// reset to the default values using this method. Anything that is **not a
    /// dynamic state of the system (e.g. tunable parameters) should not be
    /// reset using this method.**
    fn reset_state(&mut self);

    /// Reset the dynamic states, as well as any other parameters into its default
    /// values
    ///
    /// This is a hard reset on the simulation, instead of restarting the
    /// simulation with same parameters.
    fn reset_all(&mut self);
}

/// Trait to allow visually representing simulation (simulation graphics + GUI)
pub trait Draw {
    /// Draw the simulation onto a 2D scene
    fn draw(&self, plot_ui: &mut PlotUi);
    /// Draw any GUI elements to interact with the simulation
    fn options_ui(&mut self, ui: &mut Ui);
    /// Draw time-domain plot
    fn plot(&self, plot_ui: &mut PlotUi) {}
}

/// Super-trait for objects which implement both [`Simulate`] and [`Draw`]
///
/// This trait is required in order to simulate and draw using [`egui`].
pub trait SimulateEgui: Simulate + Draw {
    /// A downcast method to access another simulation object as a generic [`Simulate`]
    /// object, instead of [`SimulateEgui`].
    ///
    /// The primary usecase for this method is for state synchronization between
    /// multiple simulations via [`Simulate::match_state_with`]
    fn as_base(&self) -> &dyn Simulate;
}

impl<T> SimulateEgui for T
where
    T: Simulate + Draw,
{
    fn as_base(&self) -> &dyn Simulate {
        self
    }
}

/// A concrete type for containing simulations and executing them
pub struct Simulator {
    /// An array of simulations to be shown on the same window and simulated
    /// together with a uniform time step.
    simulations: Vec<Box<dyn SimulateEgui>>,
    /// Current simulation time in seconds.
    time: f32,
    /// The speed with which to execute the simulation. This is actually a
    /// multiplier to indicate how many times to call [`step`](Simulate::step) when
    /// [`update`](Self::update) is called.
    sim_speed: usize,
    /// Settings to indicate whether to show the graph of simulation signals
    show_graph: bool,
}

impl Default for Simulator {
    fn default() -> Self {
        Self {
            simulations: vec![Box::new(InvertedPendulum::default())],
            time: 0.0,
            sim_speed: 2,
            show_graph: false,
        }
    }
}

impl Simulator {
    /// Update the simulation for a single time step
    pub fn update(&mut self) {
        let dt = 0.01;
        self.time += dt;
        self.simulations
            .iter_mut()
            .for_each(|sim| (0..self.sim_speed).for_each(|_| sim.step(dt)));
    }

    /// Reset the states of all simulations within the currrent [`Simulator`]
    pub fn reset_state(&mut self) {
        self.simulations
            .iter_mut()
            .for_each(|sim| sim.reset_state());
    }

    /// Add a new simulation instance to the current [`Simulator`]
    pub fn add(&mut self) {
        let id = self.simulations.len() + 1;
        self.simulations.push(Box::new(InvertedPendulum::new(id)));
    }

    /// Draw 2D graphics and GUI elements related to simulation
    fn scene_ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.collapsing("Instructions", |ui| {
                ui.label("Pan by dragging, or scroll (+ shift = horizontal).");
                ui.label("Box zooming: Right click to zoom in and zoom out using a selection.");
                if cfg!(target_arch = "wasm32") {
                    ui.label("Zoom with ctrl / ⌘ + pointer wheel, or with pinch gesture.");
                } else if cfg!(target_os = "macos") {
                    ui.label("Zoom with ctrl / ⌘ + scroll.");
                } else {
                    ui.label("Zoom with ctrl + scroll.");
                }
                ui.label("Reset view with double-click.");
                // ui.add(crate::egui_github_link_file!());
            });
        });

        ui.checkbox(&mut self.show_graph, "Show Graph");

        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("Restart").clicked() {
                self.simulations
                    .iter_mut()
                    .for_each(|sim| sim.reset_state());

                // Use the first simulation's states to sync with the rest of simulations
                let (first, rest) = self.simulations.split_at_mut(1);
                if let Some(first) = first.first() {
                    rest.iter_mut()
                        .for_each(|sim| sim.match_state_with(first.as_base()));
                }
            }
            if ui.button("Reset All").clicked() {
                self.simulations.iter_mut().for_each(|sim| sim.reset_all());
            }
            if ui.button("Add New").clicked() {
                self.add();
            }
        });

        ui.horizontal(|ui| {
            self.simulations.iter_mut().for_each(|sim| {
                sim.options_ui(ui);
            });
        });

        let plot = Plot::new("Scene")
            .legend(Legend::default().position(Corner::RightTop))
            .show_x(false)
            .show_y(false)
            .data_aspect(1.0);

        ui.separator();

        plot.show(ui, |plot_ui| {
            self.simulations
                .iter_mut()
                .for_each(|sim| sim.draw(plot_ui));
        });
    }

    fn graph_ui(&mut self, ui: &mut Ui) {
        self.simulations.iter_mut().for_each(|sim| {
            Plot::new("Plot")
                .legend(Legend::default().position(Corner::RightTop))
                .data_aspect(1.0)
                .show(ui, |plot_ui| sim.plot(plot_ui));
        });
    }
}

impl View for Simulator {
    fn name(&self) -> &'static str {
        "Simulator"
    }

    fn show(&mut self, ctx: &Context, open: &mut bool) {
        Window::new(self.name())
            .open(open)
            .default_size(vec2(400.0, 400.0))
            .vscroll(false)
            .show(ctx, |ui| self.scene_ui(ui));

        if self.show_graph {
            Window::new(format!("{} {}", self.name(), "Plot"))
                .open(open)
                .default_size(vec2(400.0, 400.0))
                .vscroll(false)
                .show(ctx, |ui| self.graph_ui(ui));
        }
    }

    fn ui(&mut self, ui: &mut Ui) {
        if self.show_graph {
            egui::SidePanel::right("right_panel")
                .resizable(true)
                .default_width(150.0)
                .width_range(80.0..=500.0)
                .show_inside(ui, |ui| {
                    self.graph_ui(ui);
                });
            egui::CentralPanel::default().show_inside(ui, |ui| {
                self.scene_ui(ui);
            });
        } else {
            self.scene_ui(ui);
        }
    }
}
