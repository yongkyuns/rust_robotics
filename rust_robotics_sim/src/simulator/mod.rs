pub mod pendulum;

use crate::prelude::*;
use pendulum::InvertedPendulum;

use egui::{plot::PlotUi, *};
use plot::{Corner, Legend, Plot};

pub trait Simulate {
    fn get_state(&self) -> &dyn std::any::Any;
    fn match_state_with(&mut self, other: &dyn Simulate);
    fn step(&mut self, dt: f32);
    fn reset_state(&mut self);
    fn reset_all(&mut self);
}

pub trait Draw {
    fn draw(&self, plot_ui: &mut PlotUi);
    fn options_ui(&mut self, ui: &mut Ui);
}

pub trait SimulateEgui: Simulate + Draw {
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

pub struct Simulator {
    simulations: Vec<Box<dyn SimulateEgui>>,
    time: f32,
    sim_speed: usize,
}

impl Default for Simulator {
    fn default() -> Self {
        Self {
            simulations: vec![Box::new(InvertedPendulum::default())],
            time: 0.0,
            sim_speed: 2,
        }
    }
}

impl Simulator {
    pub fn update(&mut self) {
        let dt = 0.01;
        self.time += dt;
        self.simulations
            .iter_mut()
            .for_each(|sim| (0..self.sim_speed).for_each(|_| sim.step(dt)));
    }

    pub fn reset_state(&mut self) {
        self.simulations
            .iter_mut()
            .for_each(|sim| sim.reset_state());
    }
    pub fn add(&mut self) {
        let id = self.simulations.len() + 1;
        self.simulations.push(Box::new(InvertedPendulum::new(id)));
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
            .show(ctx, |ui| self.ui(ui));
    }

    fn ui(&mut self, ui: &mut Ui) {
        // let rect = Rectangle::new()
        //     .with_width(4.0)
        //     .with_height(2.0)
        //     .with_angle(std::f64::consts::PI / 4.0)
        //     .into_polygon();
        // // rect.color(color)

        // let n = 100;
        // let mut sin_values: Vec<_> = (0..=n)
        //     .map(|i| remap(i as f64, 0.0..=n as f64, -TAU..=TAU))
        //     .map(|i| Value::new(i, i.sin()))
        //     .collect();

        // let line = Line::new(Values::from_values(sin_values.split_off(n / 2))).fill(0.0);
        // let polygon = Ellipse::new()
        //     .with_width(4.0)
        //     .with_height(2.0)
        //     .with_angle(std::f64::consts::PI / 4.0)
        //     .into_polygon();

        // let circle = Circle::new().with_radius(2.0).into_polygon();

        // let points = Points::new(Values::from_values(sin_values))
        //     .stems(-1.5)
        //     .radius(1.0);

        // let arrows = {
        //     let pos_radius = 8.0;
        //     let tip_radius = 7.0;
        //     let arrow_origins = Values::from_parametric_callback(
        //         |t| (pos_radius * t.sin(), pos_radius * t.cos()),
        //         0.0..TAU,
        //         36,
        //     );
        //     let arrow_tips = Values::from_parametric_callback(
        //         |t| (tip_radius * t.sin(), tip_radius * t.cos()),
        //         0.0..TAU,
        //         36,
        //     );
        //     Arrows::new(arrow_origins, arrow_tips)
        // };

        // let texture: &egui::TextureHandle = self.texture.get_or_insert_with(|| {
        //     ui.ctx()
        //         .load_texture("plot_demo", egui::ColorImage::example())
        // });
        // let image = PlotImage::new(
        //     texture,
        //     Value::new(0.0, 10.0),
        //     5.0 * vec2(texture.aspect_ratio(), 1.0),
        // );

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

        ui.separator();
        ui.horizontal(|ui| {
            if ui.button("Restart").clicked() {
                self.simulations
                    .iter_mut()
                    .for_each(|sim| sim.reset_state());

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
                ui.separator();
                sim.options_ui(ui);
            });
        });

        let plot = Plot::new("items_demo")
            .legend(Legend::default().position(Corner::RightBottom))
            .show_x(false)
            .show_y(false)
            .data_aspect(1.0);

        ui.separator();

        plot.show(ui, |plot_ui| {
            self.simulations
                .iter_mut()
                .for_each(|sim| sim.draw(plot_ui));

            // plot_ui.polygon(rect.name("Rectangle"));
            // plot_ui.hline(HLine::new(9.0).name("Lines horizontal"));
            // plot_ui.hline(HLine::new(-9.0).name("Lines horizontal"));
            // plot_ui.vline(VLine::new(9.0).name("Lines vertical"));
            // plot_ui.vline(VLine::new(-9.0).name("Lines vertical"));
            // plot_ui.line(line.name("Line with fill"));
            // plot_ui.polygon(polygon.name("Convex polygon"));
            // plot_ui.polygon(circle.name("Circle"));
            // plot_ui.points(points.name("Points with stems"));
            // plot_ui.text(Text::new(Value::new(-3.0, -3.0), "wow").name("Text"));
            // plot_ui.text(Text::new(Value::new(-2.0, 2.5), "so graph").name("Text"));
            // plot_ui.text(Text::new(Value::new(3.0, 3.0), "much color").name("Text"));
            // plot_ui.text(Text::new(Value::new(2.5, -2.0), "such plot").name("Text"));
            // plot_ui.image(image.name("Image"));
            // plot_ui.arrows(arrows.name("Arrows"));
            // draw_cart(
            //     plot_ui,
            //     self.pendulum.x_position() as f64,
            //     self.pendulum.rod_angle() as f64,
            // );

            // self.cart.plot(
            //     plot_ui,
            //     self.pendulum.x_position() as f64,
            //     self.pendulum.rod_angle() as f64,
            // );
        });
        // .response
    }
}
