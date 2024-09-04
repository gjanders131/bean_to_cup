use egui::{pos2, Order, Ui, Window};

/// Popup Functions
pub trait Popup {
    /// `&'static` so we can also use it as a key to store open/close state.
    fn name(&self) -> &'static str;

    fn is_enabled(&mut self) -> bool;

    /// Show windows, etc
    fn show(&mut self, ctx: &egui::Context, open: &mut bool);

    fn ui(&mut self, ui: &mut Ui);
}
