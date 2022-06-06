use crate::plot_demo::PlotDemo;
use crate::simulator::Simulator;
use crate::View;

use eframe::egui;

#[derive(Default)]
pub struct State {
    demo: PlotDemo,
    sim: Simulator,
}

#[derive(Default)]
pub struct App {
    state: State,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::widgets::global_dark_light_mode_switch(ui);
            ui.heading("Rust Robotics");

            if ui.button("Reset").clicked() {
                self.state.sim.reset();
            }
        }); // just to paint a background for the windows to be on top of. Needed on web because of https://github.com/emilk/egui/issues/1548

        self.state.sim.update();

        self.state.demo.show(ctx, &mut true);
        self.state.sim.show(ctx, &mut true);
    }
}
