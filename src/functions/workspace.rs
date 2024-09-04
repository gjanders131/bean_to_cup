use std::fs;

use egui::{pos2, Order, Ui, Widget, Window};
use serde::{Deserialize, Serialize};

use super::{Categories, Popup, TemplateApp};

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Workspace {
    pub name: String,
    pub path: String,
    pub dir_categories: Categories,
    #[serde(skip)]
    pub ui_create_workspace: Box<dyn Popup>,
}

impl Workspace {
    pub fn new(name: String, path: String, dir_categories: Categories) -> Workspace {
        Workspace {
            name,
            path,
            dir_categories,
            ui_create_workspace: Box::<CreateWorkspace>::default(),
        }
    }

    pub fn open_workspace() -> Result<Workspace, String> {
        let file = rfd::FileDialog::new().pick_file();
        match file {
            Some(_) => (),
            None => return Err("No file selected".to_string()),
        }
        let content: Result<Workspace, String> =
            serde_json::from_str(std::fs::read_to_string(file.unwrap()).unwrap().as_str())
                .map_err(|err| err.to_string());
        return content;
    }

    pub fn save_workspace(&self) -> Result<(), String> {
        let json_content = serde_json::to_value(&self).map_err(|err| err.to_string())?;
        fs::write(
            format!("{}{}{}{}", &self.path, "/", &self.name, ".json"),
            serde_json::to_string_pretty(&json_content).expect("Unable to write JSON to string"),
        )
        .map_err(|err| err.to_string())
    }
}

pub fn ui_workspace(ui: &mut Ui, app: &mut TemplateApp, ctx: &egui::Context) {
    ui.vertical(|ui| {
        ui.heading("Workspace");
        ui.horizontal(|ui| {
            if ui.button("Open Workspace").clicked() {
                match Workspace::open_workspace() {
                    Ok(workspace) => app.workspace = workspace,
                    Err(err) => println!("{}", err),
                }
            }
            if ui.button("Create Workspace").clicked() {
                let open = &mut true;
                app.create_workspace.show(ctx, open);
            }
        });
    });
}

#[derive(Default, Serialize, Deserialize)]
pub struct CreateWorkspace {
    name: String,
    path: String,
}

impl Popup for CreateWorkspace {
    fn name(&self) -> &'static str {
        "Create a new workspace"
    }

    fn ui(&mut self, ui: &mut Ui) {
        ui.label("Enter a name for your new workspace:");
        ui.text_edit_singleline(&mut self.name);
        if ui.button("Workspace Location").clicked() {
            match rfd::FileDialog::new().pick_folder() {
                Some(folder) => self.path = folder.to_str().unwrap().to_string(),
                None => println!("No Folder Selected"),
            }
        }
        ui.label(&self.name);
        ui.label(&self.path);
    }

    fn is_enabled(&mut self) -> bool {
        true
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(self.name())
            .default_width(300.0)
            .default_height(100.0)
            .open(open)
            .resizable([false, false])
            .movable(true)
            .collapsible(false)
            .default_pos(pos2(0.0, 0.0))
            .order(Order::Foreground)
            .show(ctx, |ui| {
                self.ui(ui);
            });
    }
}
