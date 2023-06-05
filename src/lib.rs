pub mod components;

use components::{Component, Input, Ports};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use vizia::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct ComponentStore {
    pub store: Vec<Box<dyn Component>>,
}

// a mapping (id -> Ports)
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
            let (id, ports) = c.get_id_ports();
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

// a mapping (id -> index)
// where index is the start index in the LensValues vector
// e.g., `mux1` starts at index 15, then its
// select input is index 15
// the first input is index 16
// the second input is index 17, etc.
#[derive(Debug)]
pub struct IdStartIndex(pub HashMap<String, usize>);

// a mapping (id -> Input)
// where Input holds the id and index for the connected component
#[derive(Debug)]
pub struct IdInput(pub HashMap<String, Input>);

impl SimState {
    pub fn new(component_store: ComponentStore) -> Self {
        let mut sim_state = SimState {
            lens_values: LensValues { values: vec![] },
            id_ports: component_store.to_id_ports(),
        };

        let mut id_start_index = IdStartIndex(HashMap::new());

        let mut id_input = IdInput(HashMap::new()); // allocate storage for lensed outputs
        for c in component_store.store {
            let (id, ports) = c.get_id_ports();
            // start index for outputs related to component
            id_start_index
                .0
                .insert(id.clone(), sim_state.lens_values.values.len().clone());

            // build topological dependency
            for input in ports.inputs {
                id_input.0.insert(id.clone(), input);
            }

            for _ in ports.outputs {
                // create the value with a default to 0
                sim_state.lens_values.values.push(0);
            }
        }

        println!("id input {:?}", id_input);
        sim_state
    }
}

// Simulator implementation
impl SimState {
    pub fn get(&self, index: usize) -> u32 {
        *self.lens_values.values.get(index).unwrap()
    }

    pub fn set(&mut self, index: usize, value: u32) {
        let val_ref = self.lens_values.values.get_mut(index).unwrap();
        *val_ref = value
    }
}

// impl Model for SimState {}
