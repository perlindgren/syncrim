use crate::common::OutputType;
use crate::common::{Component, Input, Ports};
use crate::component_store::ComponentStore;
use petgraph::algo;
use petgraph::algo::toposort;
use petgraph::Direction::Outgoing;
use petgraph::Graph;
use std::collections::HashMap;

use vizia::prelude::*;

#[derive(Lens, Debug, Clone)]
pub struct LensValues {
    pub values: Vec<u32>,
}

#[derive(Debug)]
pub struct SimState {
    pub lens_values: LensValues,
    // pub id_ports: IdPorts,
}

// a mapping (id -> index)
// where index is the start index in the LensValues vector
// e.g., `mux1` starts at index 15, then its
// select input is index 15
// the first input is index 16
// the second input is index 17, etc.
#[derive(Debug)]
pub struct IdStartIndex(pub HashMap<String, usize>);

pub struct IdComponent(pub HashMap<String, Box<dyn Component>>);

impl SimState {
    pub fn new(component_store: ComponentStore) -> Self {
        let mut sim_state = SimState {
            lens_values: LensValues { values: vec![] },
            // id_ports: component_store.to_id_ports(),
        };

        let mut id_start_index = IdStartIndex(HashMap::new());

        let mut id_component = IdComponent(HashMap::new());

        // allocate storage for lensed outputs
        for c in component_store.store {
            let (id, ports) = c.get_id_ports();

            println!("id {}, ports {:?}", id, ports);
            // start index for outputs related to component
            id_start_index
                .0
                .insert(id.clone(), sim_state.lens_values.values.len().clone());

            id_component.0.insert(id, c);

            for _ in ports.outputs {
                // create the value with a default to 0
                sim_state.lens_values.values.push(0);
            }
        }

        println!("---");

        for (id, _) in &id_component.0 {
            println!("id {}", id);
        }

        let mut graph = Graph::<_, (), petgraph::Directed>::new();
        let mut id_node = HashMap::new();

        // insert nodes
        for (id, _) in &id_component.0 {
            let node = graph.add_node(id);
            id_node.insert(id, node);
        }
        println!("id_node {:?}", id_node);

        // insert edges
        for (to_id, c) in &id_component.0 {
            let to_component = id_component.0.get(to_id).unwrap();
            let (_, ports) = to_component.get_id_ports();

            println!("to_id :{}, ports: {:?}", to_id, ports);

            if ports.out_type == OutputType::Combinatorial {
                let to_node = id_node.get(to_id).unwrap();
                let (_, ports) = c.get_id_ports();
                for in_port in &ports.inputs {
                    let from_id = &in_port.id;

                    let from_node = id_node.get(from_id).unwrap();
                    graph.add_edge(from_node.clone(), to_node.clone(), ());
                    println!(
                        "add_edge {}:{:?} -> {}:{:?}",
                        from_id, from_node, to_id, to_node
                    );
                }
            }
        }

        // topological order
        let top = toposort(&graph, None);
        println!("--- top \n{:?}", top);

        // println!("id input {:?}", id_input);
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
