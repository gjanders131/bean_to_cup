use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Categories {
    pub category: HashMap<String, Vec<String>>,
}

impl Categories {
    pub fn new() -> Categories {
        Categories {
            category: HashMap::new(),
        }
    }

    pub fn add_category(&mut self, category: &str, sub_categories: Vec<&str>) {
        self.category.insert(
            category.to_string(),
            sub_categories.iter().map(|x| x.to_string()).collect(),
        );
    }

    pub fn add_sub_category(&mut self, category: &str, sub_category: &str) {
        self.category
            .get_mut(&category.to_string())
            .unwrap()
            .push(sub_category.to_string());
    }

    pub fn remove_category(&mut self, category: &str) {
        self.category.remove(&category.to_string());
    }

    pub fn remove_sub_category(&mut self, category: &str, sub_category: &str) {
        let sub_categories = self.category.get_mut(&category.to_string()).unwrap();
        let index = sub_categories
            .iter()
            .position(|x| x == sub_category)
            .unwrap();
        sub_categories.remove(index);
    }

    pub fn get_sub_categories(&self, category: &str) -> Option<&Vec<String>> {
        self.category.get(&category.to_string())
    }

    pub fn get_all_categories(&self) -> &HashMap<String, Vec<String>> {
        &self.category
    }
}
