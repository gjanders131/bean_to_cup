mod asset;
mod categories;
mod workspace;

use categories::Categories;
use eframe::egui;
use workspace::{ui_workspace, Workspace};

fn main() -> eframe::Result {
    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Bean To Cup",
        options,
        Box::new(|cc| {
            // This gives us image support:
            // egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<App>::default())
        }),
    )
}

struct App {
    workspace: Workspace,
}

impl Default for App {
    fn default() -> Self {
        Self {
            workspace: Workspace::new("".to_string(), "".to_string(), Categories::new()),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui_workspace(ui, self);
            ui.label(&self.workspace.name);
        });
    }
}
