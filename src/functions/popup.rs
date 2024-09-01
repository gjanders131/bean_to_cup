use egui::Ui;

/// Popup Functions
pub trait Popup {
    /// `&'static` so we can also use it as a key to store open/close state.
    fn name(&self) -> &'static str;

    fn is_enabled(&mut self) -> bool;

    /// Show windows, etc
    fn show(&mut self, ctx: &egui::Context) {
        egui::Window::new(self.name())
            .default_width(300.0)
            .default_height(100.0)
            .open(&mut self.is_enabled())
            .resizable([false, false])
            .movable(true)
            .collapsible(false)
            .show(ctx, |ui| {
                self.ui(ui);
            });
    }

    fn ui(&mut self, ui: &mut Ui);
}
