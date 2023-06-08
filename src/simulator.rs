use crate::common::{Component, ComponentStore, Input, OutputType, SimState, Simulator};
use petgraph::{algo::toposort, Graph};
use std::collections::HashMap;
// use vizia::prelude::*;

pub struct IdComponent(pub HashMap<String, Box<dyn Component>>);

impl Simulator {
    pub fn new(component_store: &ComponentStore) -> (Self, SimState) {
        let mut lens_values = vec![];

        let mut id_start_index = HashMap::new();
        let mut id_component = HashMap::new(); // IdComponent(HashMap::new());

        // allocate storage for lensed outputs
        for c in &component_store.store {
            let (id, ports) = c.get_id_ports();

            println!("id {}, ports {:?}", id, ports);
            // start index for outputs related to component
            id_start_index.insert(id.clone(), lens_values.len());

            id_component.insert(id, c);

            // create placeholder for output
            #[allow(clippy::same_item_push)]
            for _ in ports.outputs {
                // create the value with a default to 0
                lens_values.push(0);
            }
        }

        println!("---");

        for id in id_component.keys() {
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
                    graph.add_edge(*from_node, *to_node, ());
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

        let mut ordered_components = vec![];
        for node in &top {
            #[allow(clippy::clone_double_ref)]
            let c = (**node_comp.get(node).unwrap()).clone();
            ordered_components.push(c);
        }

        println!("--- eval");

        (
            Simulator {
                id_start_index,
                ordered_components,
            },
            SimState { lens_values },
        )
    }
}

// Simulator implementation
impl Simulator {
    pub fn get(&self, sim_state: &SimState, index: usize) -> u32 {
        sim_state.lens_values[index]
    }

    // get input value
    pub fn get_input_val(&self, sim_state: &SimState, input: &Input) -> u32 {
        let start_index = *self.id_start_index.get(&input.id).unwrap();
        self.get(sim_state, start_index + input.index)
    }

    // get start index by id
    pub fn get_id_start_index(&self, id: &str) -> usize {
        *self.id_start_index.get(id).unwrap()
    }

    // set value by index
    pub fn set(&self, sim_state: &mut SimState, index: usize, value: u32) {
        let val_ref = &mut sim_state.lens_values[index];
        *val_ref = value
    }

    // set value by id and offset (index)
    // todo: maybe better by Output
    pub fn set_id_index(&self, sim_state: &mut SimState, id: &str, index: usize, value: u32) {
        let start_index = self.get_id_start_index(id);
        self.set(sim_state, start_index + index, value);
    }

    // iterate over the evaluators
    pub fn clock(&self, sim_state: &mut SimState) {
        for component in &self.ordered_components {
            component.evaluate(self, sim_state);
        }
    }
}
