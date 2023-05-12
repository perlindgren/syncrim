pub mod components;

use serde::{Deserialize, Serialize};
use vizia::prelude::*;

use components::{Component, Input, Ports};

#[derive(Serialize, Deserialize)]
pub struct ComponentStore {
    pub store: Vec<Box<dyn Component>>,
}

use std::collections::HashMap;
type IdPorts = HashMap<String, Ports>;

impl ComponentStore {
    pub fn load(json: &str) -> Self {
        serde_json::from_str(&json).unwrap()
    }

    pub fn to_(&self) {
        self.store.iter().for_each(|c| c.to_());
    }

    pub fn to_id_ports(&self) -> IdPorts {
        let mut id_ports = HashMap::new();
        self.store.iter().for_each(|c| {
            let (id, ports) = c.to_ports();
            id_ports.insert(id, ports);
        });
        id_ports
    }
}

#[derive(Lens, Debug, Clone)]
pub struct LensValues {
    pub values: Vec<u32>,
}

#[derive(Debug)]
pub struct SimState {
    pub lens_values: LensValues,
    pub id_ports: IdPorts,
}

#[derive(Debug)]
pub struct IdIndex(pub HashMap<String, usize>);

#[derive(Debug)]
pub struct IdId(pub HashMap<String, Input>);

impl SimState {
    pub fn new(component_store: ComponentStore) -> Self {
        let mut sim_state = SimState {
            lens_values: LensValues { values: vec![] },
            id_ports: component_store.to_id_ports(),
        };

        let mut id_index = IdIndex(HashMap::new());

        let mut id_id = IdId(HashMap::new()); // allocate storage for lensed outputs
        for c in component_store.store {
            let (id, ports) = c.to_ports();
            // start index for outputs related to component
            id_index
                .0
                .insert(id.clone(), sim_state.lens_values.values.len().clone());

            // build topological dependency
            for input in ports.inputs {
                id_id.0.insert(id.clone(), input);
            }

            for _ in ports.outputs {
                sim_state.lens_values.values.push(0);
            }
        }

        println!("id id {:?}", id_id);
        sim_state
    }
}

// impl SimState {
//     pub fn get(&self, index: usize) -> u32 {
//         *self.values.get(index).unwrap()
//     }

//     pub fn set(&mut self, index: usize, value: u32) {
//         let val_ref = self.values.get_mut(index).unwrap();
//         *val_ref = value
//     }
// }

// impl Model for SimState {}
