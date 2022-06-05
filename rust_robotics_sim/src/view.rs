/// Something to view in the demo windows
pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);

    /// `&'static` so we can also use it as a key to store open/close state.
    fn name(&self) -> &'static str;

    /// Show windows, etc
    fn show(&mut self, ctx: &egui::Context, open: &mut bool);
}
