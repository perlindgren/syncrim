use crate::common::{Component, ComponentStore, LensValues, OutputType, SimState};

use petgraph::{algo::toposort, Graph};
use std::collections::HashMap;
use vizia::prelude::*;

// a mapping (id -> index)
// where index is the start index in the LensValues vector
// e.g., `mux1` starts at index 15, then its
// select input is index 15
// the first input is index 16
// the second input is index 17, etc.
#[derive(Debug)]
pub struct IdStartIndex(pub HashMap<String, usize>);

pub struct IdComponent(pub HashMap<String, Box<dyn Component>>);

impl<'a> SimState<'a> {
    pub fn new(component_store: ComponentStore) -> Self {
        let mut lens_values = LensValues { values: vec![] };

        let mut id_start_index = IdStartIndex(HashMap::new());

        let mut id_component = HashMap::new(); // IdComponent(HashMap::new());

        // allocate storage for lensed outputs
        for c in &component_store.store {
            let (id, ports) = c.get_id_ports();

            println!("id {}, ports {:?}", id, ports);
            // start index for outputs related to component
            id_start_index
                .0
                .insert(id.clone(), lens_values.values.len().clone());

            id_component.insert(id, c);

            for _ in ports.outputs {
                // create the value with a default to 0
                lens_values.values.push(0);
            }
        }

        println!("---");

        for (id, _) in &id_component {
            println!("id {}", id);
        }

        let mut graph = Graph::<_, (), petgraph::Directed>::new();
        let mut id_node = HashMap::new();
        let mut node_comp = HashMap::new();

        // insert nodes
        for (id, c) in &id_component {
            let node = graph.add_node(id);
            id_node.insert(id, node);
            node_comp.insert(node, c);
        }
        println!("id_node {:?}", id_node);

        for (node, c) in &node_comp {
            println!("node {:?}, comp_id {:?}", node, c.get_id_ports());
        }

        // insert edges
        for (to_id, c) in &id_component {
            let to_component = id_component.get(to_id).unwrap();
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
        let top = toposort(&graph, None).unwrap();
        println!("--- top \n{:?}", top);

        let mut eval = vec![];
        for node in &top {
            let c = node_comp.get(node).unwrap().clone();
            eval.push(c);
        }

        println!("--- eval");

        for c in &eval {
            let (id, _) = c.get_id_ports();
            println!("id {}", id);
        }

        SimState {
            lens_values,
            component_store,
            eval: vec![],
        }
    }
}

// Simulator implementation
impl<'a> SimState<'a> {
    pub fn get(&self, index: usize) -> u32 {
        *self.lens_values.values.get(index).unwrap()
    }

    pub fn set(&mut self, index: usize, value: u32) {
        let val_ref = self.lens_values.values.get_mut(index).unwrap();
        *val_ref = value
    }
}
