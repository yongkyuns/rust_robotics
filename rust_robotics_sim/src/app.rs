use crate::simulator::Simulator;
use crate::View;

use eframe::egui;

#[derive(Default)]
pub struct State {
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
        self.state.sim.update();
        egui::CentralPanel::default().show(ctx, |_ui| {
            // egui::widgets::global_dark_light_mode_switch(ui);
            // egui::widgets::global_dark_light_mode_buttons(ui);
            ctx.set_visuals(egui::Visuals::dark());
            // ui.heading("Rust Robotics");

            self.state.sim.show(ctx, &mut true);
        }); // just to paint a background for the windows to be on top of. Needed on web because of https://github.com/emilk/egui/issues/1548
    }
}
