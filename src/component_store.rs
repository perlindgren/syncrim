use crate::common::Component;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ComponentStore {
    pub store: Vec<Box<dyn Component>>,
}

impl ComponentStore {
    pub fn load(json: &str) -> Self {
        serde_json::from_str(&json).unwrap()
    }

    pub fn to_(&self) {
        self.store.iter().for_each(|c| c.to_());
    }
}
