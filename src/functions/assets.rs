use super::Categories;

pub struct File {
    pub name: String,
    pub ext: String,
    pub path: String,
}
pub struct Asset {
    pub name: String,
    pub path: String,
    pub dir_categories: Vec<String>,
    pub asset_categories: Categories,
    pub files: Vec<File>,
}

impl Asset {
    fn new(name: String, path: String) -> Asset {
        Asset {
            name,
            path,
            dir_categories: Vec::new(),
            asset_categories: Categories::new(),
            files: Vec::new(),
        }
    }
}

pub struct AssetList {
    pub assets: Vec<Asset>,
}

impl AssetList {
    pub fn new() -> AssetList {
        AssetList { assets: Vec::new() }
    }

    pub fn add_asset(&mut self, name: String, path: String) {
        self.assets.push(Asset::new(name, path));
    }

    pub fn remove_asset(&mut self, name: &str) {
        let index = self.assets.iter().position(|x| x.name == name).unwrap();
        self.assets.remove(index);
    }

    pub fn get_asset(&self, name: &str) -> Option<&Asset> {
        self.assets.iter().find(|x| x.name == name)
    }

    pub fn get_all_assets(&self) -> &Vec<Asset> {
        &self.assets
    }
}
