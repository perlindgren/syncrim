use crate::common::ComponentStore;

use std::{fmt::Display, fs::File, io::prelude::*, path::PathBuf};

use log::*;

#[derive(Debug)]
pub enum ComponentStoreLoadError {
    Json(serde_json::Error),
    Io(std::io::Error, PathBuf),
}
impl Display for ComponentStoreLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComponentStoreLoadError::Json(error) => {
                write!(f, "Error while decoding json: {}", error)
            }
            ComponentStoreLoadError::Io(error, path) => {
                write!(f, "Error while reading file {:?} : {}", path, error)
            }
        }
    }
}

impl std::error::Error for ComponentStoreLoadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ComponentStoreLoadError::Json(error) => Some(error),
            ComponentStoreLoadError::Io(error, _) => Some(error),
        }
    }
}

impl From<serde_json::Error> for ComponentStoreLoadError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

impl ComponentStore {
    pub fn load(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn load_file(path: &PathBuf) -> Result<Self, ComponentStoreLoadError> {
        let mut file =
            File::open(path).map_err(|err| ComponentStoreLoadError::Io(err, path.clone()))?;
        let mut json = String::new();
        file.read_to_string(&mut json)
            .map_err(|err| ComponentStoreLoadError::Io(err, path.clone()))?;

        let cs = ComponentStore::load(&json)?;
        Ok(cs)
    }

    pub fn save_file(&self, path: &PathBuf) {
        let json = serde_json::to_string_pretty(self).unwrap();
        trace!("json: {}", json);
        trace!("path {:?}", path);
        let mut file = File::create(path).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }

    pub fn to_(&self) {
        self.store.iter().for_each(|c| {
            c.to_();
        })
    }
}
