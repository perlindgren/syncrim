use crate::common::ComponentStore;

use std::fs::File;
use std::io::prelude::*;

impl ComponentStore {
    pub fn load(json: &str) -> Self {
        serde_json::from_str(json).unwrap()
    }

    pub fn load_file(path: &str) -> Self {
        let mut file = File::open(path).unwrap();
        let mut json = String::new();
        file.read_to_string(&mut json).unwrap();

        ComponentStore::load(&json)
    }

    pub fn save_file(&self, path: &str) {
        let json = serde_json::to_string(self).unwrap();
        println!("json: {}", json);
        let mut file = File::create(path).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }

    pub fn to_(&self) {
        self.store.iter().for_each(|c| c.to_());
    }
}
