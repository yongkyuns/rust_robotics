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
        }); // just to paint a background for the windows to be on top of. Needed on web because of https://github.com/emilk/egui/issues/1548

        self.state.demo.show(ctx, &mut true);
        self.state.sim.show(ctx, &mut true);

        // egui::CentralPanel::default().show(ctx, |ui| {
        //     ui.heading("Hello World!");
        // });

        // egui::TopBottomPanel::top("wrap_app_top_bar").show(ctx, |ui| {
        //     egui::trace!(ui);
        //     self.bar_contents(ui, frame);
        // });

        // self.state.backend_panel.update(ctx, frame);

        // if self.state.backend_panel.open || ctx.memory().everything_is_visible() {
        //     egui::SidePanel::left("backend_panel").show(ctx, |ui| {
        //         self.state.backend_panel.ui(ui, frame);

        //         ui.separator();

        //         ui.horizontal(|ui| {
        //             if ui
        //                 .button("Reset egui")
        //                 .on_hover_text("Forget scroll, positions, sizes etc")
        //                 .clicked()
        //             {
        //                 *ui.ctx().memory() = Default::default();
        //             }

        //             if ui.button("Reset everything").clicked() {
        //                 self.state = Default::default();
        //                 *ui.ctx().memory() = Default::default();
        //             }
        //         });
        //     });
        // }

        // let mut found_anchor = false;

        // let selected_anchor = self.state.selected_anchor.clone();
        // for (_name, anchor, app) in self.apps_iter_mut() {
        //     if anchor == selected_anchor || ctx.memory().everything_is_visible() {
        //         app.update(ctx, frame);
        //         found_anchor = true;
        //     }
        // }

        // if !found_anchor {
        //     self.state.selected_anchor = "demo".into();
        // }

        // self.state.backend_panel.end_of_frame(ctx);

        // self.ui_file_drag_and_drop(ctx);
    }
}
