use std::fs;

use egui::{InnerResponse, Ui};
use serde::{Deserialize, Serialize};

use crate::{categories::Categories, App};

#[derive(Debug, Serialize, Deserialize)]
pub struct Workspace {
    pub name: String,
    pub path: String,
    pub dir_categories: Categories,
}

impl Workspace {
    pub fn new(name: String, path: String, dir_categories: Categories) -> Workspace {
        Workspace {
            name,
            path,
            dir_categories,
        }
    }

    pub fn open_workspace() -> Workspace {
        let file = rfd::FileDialog::new().pick_file().unwrap();
        let content: Workspace =
            serde_json::from_str(std::fs::read_to_string(file).unwrap().as_str()).unwrap();
        Workspace::new(content.name, content.path, content.dir_categories)
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

pub fn ui_workspace(ui: &mut Ui, app: &mut App) {
    ui.vertical(|ui| {
        ui.heading("Workspace");
        ui.horizontal(|ui| {
            if ui.button("Open Workspace").clicked() {
                app.workspace = Workspace::open_workspace();
            }
            if ui.button("Create Workspace").clicked() {
                // save_workspace();
                todo!();
            }
        });
    });
}
