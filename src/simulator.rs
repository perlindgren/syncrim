use crate::common::{
    Component, ComponentStore, Condition, Id, Input, OutputType, Signal, SignalFmt, SignalValue,
    Simulator,
};
use log::*;
use petgraph::{
    algo::toposort,
    dot::{Config, Dot},
    Graph,
};
use std::collections::HashMap;
use std::{fs::File, io::prelude::*, path::PathBuf};

pub struct IdComponent(pub HashMap<String, Box<dyn Component>>);

// Notice:
// The topological order does not enforce any specific order of registers
// Thus registers cannot point to other registers in a cyclic fashion
// This is (likely) not occurring in practice.
//
// A solution is to evaluate register updates separately from other components
// ... but not currently implemented ...
impl Simulator {
    pub fn new(component_store: ComponentStore) -> Result<Self, &'static str> {
        for component in &component_store.store {
            component.reset();
        }
        let mut lens_values = vec![];

        let mut id_start_index = HashMap::new();
        let mut id_component = HashMap::new(); // IdComponent(HashMap::new());

        let mut id_nr_outputs = HashMap::new();
        let mut id_field_index = HashMap::new();
        // allocate storage for lensed outputs

        trace!("-- allocate storage for lensed outputs");
        for c in &component_store.store {
            trace!("{:?}", c.get_id_ports().0);
            let (id, ports) = c.get_id_ports();

            trace!("id {}, ports {:?}", id, ports);
            // start index for outputs related to component
            if id_start_index
                .insert(id.clone(), lens_values.len())
                .is_some()
            {
                panic!("Component identifier {:?} is defined twice", id);
            }

            id_component.insert(id.clone(), c);

            // create placeholder for output
            #[allow(clippy::same_item_push)]
            for (index, field_id) in ports.outputs.iter().enumerate() {
                // create the value with a default to 0
                lens_values.push(0.into());
                if id_field_index
                    .insert((id.clone(), field_id.into()), index)
                    .is_some()
                {
                    panic!("Component {:?} field {:?} is defined twice", id, field_id)
                };
            }
            id_nr_outputs.insert(id.clone(), ports.outputs.len());
        }

        let mut graph = Graph::<_, (), petgraph::Directed>::new();
        let mut id_node = HashMap::new();
        let mut node_comp = HashMap::new();

        // insert nodes
        for (id, c) in &id_component {
            let node = graph.add_node(id.to_owned());
            id_node.insert(id, node);
            node_comp.insert(node, c);
        }

        trace!("\nid_node {:?}", id_node);

        for (node, c) in &node_comp {
            trace!("node {:?}, comp_id {:?}", node, c.get_id_ports());
        }

        // insert edges
        for (to_id, c) in &id_component {
            let to_component = id_component.get(to_id).unwrap();
            let (_, ports) = to_component.get_id_ports();

            trace!("to_id :{}, ports: {:?}", to_id, ports);

            if ports.out_type == OutputType::Combinatorial {
                trace!("combinatorial, id:{}", to_component.get_id_ports().0);
                let to_node = id_node.get(to_id).unwrap();
                let (_, ports) = c.get_id_ports();
                for in_port in &ports.inputs {
                    let from_id = &in_port.input.id;
                    let from_node = id_node.get(from_id);
                    if from_node.is_none() {
                        println!("to id: {} from port {} is not connected", to_id, from_id);
                        return Err("A port left unconnected");
                    }
                    let from_node = from_node.unwrap();

                    graph.add_edge(*from_node, *to_node, ());
                    trace!(
                        "add_edge {}:{:?} -> {}:{:?}",
                        from_id,
                        from_node,
                        to_id,
                        to_node
                    );
                }
            }
        }

        // topological order
        let top =
            toposort(&graph, None).expect("Topological sort failed, your model contains loops.");
        trace!("--- topologically ordered graph \n{:?}", top);
        //two passes, first add all sequential roots
        let mut ordered_components = vec![];
        //two passes ensure the sorted list of nodes always starts with ALL of the roots
        //first push the sequential components, eg. graph roots
        for node in &top {
            #[allow(suspicious_double_ref_op)]
            let c = (**node_comp.get(node).unwrap()).clone();
            if c.get_id_ports().1.out_type == OutputType::Sequential {
                ordered_components.push(c);
            }
        }
        //then the rest...
        for node in &top {
            #[allow(suspicious_double_ref_op)]
            let c = (**node_comp.get(node).unwrap()).clone();
            if c.get_id_ports().1.out_type == OutputType::Combinatorial {
                ordered_components.push(c);
            }
        }

        let component_ids: Vec<Id> = ordered_components
            .iter()
            .map(|c| c.get_id_ports().0)
            .collect();

        trace!(
            "--- topologically ordered component identifiers \n{:?}",
            component_ids
        );

        let mut simulator = Simulator {
            cycle: 0,
            id_start_index,
            ordered_components,
            id_nr_outputs,
            id_field_index,
            sim_state: lens_values,
            history: vec![],
            component_ids,
            graph,
            running: false,
        };

        trace!("sim_state {:?}", simulator.sim_state);
        simulator.clock();
        Ok(simulator)
    }

    /// get input by index
    pub(crate) fn get(&self, index: usize) -> Signal {
        self.sim_state[index]
    }

    /// get input signal
    pub fn get_input_signal(&self, input: &Input) -> Signal {
        #[allow(unreachable_code)]
        let nr_out = *self
            .id_nr_outputs
            .get(&input.id)
            .unwrap_or_else(|| panic!("\n{:?} not found in \n{:?}", input, self.id_nr_outputs));
        let index = *self
            .id_field_index
            .get(&(input.id.clone(), input.field.clone()))
            .unwrap_or_else(|| {
                panic!(
                    "Component {:?}, field {:?} not found.",
                    input.id, input.field
                )
            });
        if index < nr_out {
            let start_index = *self.id_start_index.get(&input.id).unwrap();
            self.get(start_index + index)
        } else {
            panic!(
                "ICE: Attempt to read {:?} at index {}, where {:?} has only {} outputs.",
                input.id, index, input.id, nr_out
            )
        }
    }

    /// get input value
    pub fn get_input_value(&self, input: &Input) -> SignalValue {
        self.get_input_signal(input).get_value()
    }

    /// get input fmt
    pub fn get_input_fmt(&self, input: &Input) -> SignalFmt {
        self.get_input_signal(input).get_fmt()
    }

    /// get start index by id
    pub(crate) fn get_id_start_index(&self, id: &str) -> usize {
        *self.id_start_index.get(id).unwrap()
    }

    // set value by index
    fn set_value(&mut self, index: usize, value: SignalValue) {
        self.sim_state[index].set_value(value);
    }

    // set fmt by index
    fn set_fmt(&mut self, index: usize, fmt: SignalFmt) {
        self.sim_state[index].set_fmt(fmt);
    }

    /// set value by Id (instance) and Id (field)
    pub fn set_out_value(&mut self, id: &str, field: &str, value: impl Into<SignalValue>) {
        let index = *self
            .id_field_index
            .get(&(id.into(), field.into()))
            .unwrap_or_else(|| panic!("Component {}, field {} not found.", id, field));
        let start_index = self.get_id_start_index(id);
        let val: SignalValue = value.into();
        //trace!("id:{}, field:{}, value:{:?}", id,field, SignalValue::try_from(val).unwrap());
        self.set_value(start_index + index, val);
    }

    /// set fmt by Id (instance) and Id (field)
    pub fn set_out_fmt(&mut self, id: &str, field: &str, fmt: SignalFmt) {
        let index = *self
            .id_field_index
            .get(&(id.into(), field.into()))
            .unwrap_or_else(|| panic!("Component {}, field {} not found.", id, field));
        let start_index = self.get_id_start_index(id);
        self.set_fmt(start_index + index, fmt);
    }

    /// iterate over the evaluators and increase clock by one
    pub fn clock(&mut self) {
        // push current state
        self.history.push(self.sim_state.clone());
        trace!("cycle:{}", self.cycle);
        for component in self.ordered_components.clone() {
            //trace!("evaling component:{}", component.get_id_ports().0);
            match component.clock(self) {
                Ok(_) => {}
                Err(cond) => match cond {
                    Condition::Warning(warn) => {
                        trace!("warning {}", warn)
                    }
                    Condition::Error(err) => panic!("err {}", err),
                    Condition::Assert(assert) => {
                        error!("assertion failed {}", assert);
                        self.running = false;
                    }
                    Condition::Halt(halt) => {
                        self.running = false;
                        info!("halt {}", halt)
                    }
                },
            }
        }
        self.cycle = self.history.len();
    }

    /// free running mode until Halt condition
    pub fn run(&mut self) {
        use std::time::Instant;
        let now = Instant::now();
        while now.elapsed().as_millis() < 1000 / 30 {
            //60Hz
            if self.running {
                self.clock()
            }
        }
    }

    pub fn run_threaded(&mut self) {}

    /// stop the simulator from gui or other external reason
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// reverse simulation using history if clock > 1
    pub fn un_clock(&mut self) {
        if self.cycle > 1 {
            let state = self.history.pop().unwrap();
            // set old state
            self.sim_state = state;
            // to ensure that history length and cycle count complies
            self.cycle = self.history.len();

            for component in self.ordered_components.clone() {
                component.un_clock();
            }
        }
    }

    /// reset simulator
    pub fn reset(&mut self) {
        // The order of the following is not important
        // with the exception that self.clock() needs to be last
        self.history = vec![];
        self.cycle = 0;
        self.stop();

        self.sim_state.iter_mut().for_each(|val| *val = 0.into());

        for component in self.ordered_components.clone() {
            component.reset();
        }

        self.clock();
    }

    pub fn get_state(&self) -> bool {
        self.running
    }

    /// save as `dot` file with `.gv` extension
    pub fn save_dot(&self, path: &PathBuf) {
        let mut path = path.to_owned();
        path.set_extension("gv");
        let mut file = File::create(path).unwrap();
        let dot_string = format!(
            "{:?}",
            Dot::with_config(&self.graph, &[Config::EdgeNoLabel])
        );
        file.write_all(dot_string.as_bytes()).unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::components::*;
    use std::rc::Rc;

    #[test]
    fn test_define() {
        let cs = ComponentStore {
            store: vec![Rc::new(ProbeOut::new("po1"))],
        };

        let simulator = Simulator::new(cs).unwrap();

        assert_eq!(simulator.cycle, 1);
    }

    #[test]
    #[should_panic(expected = "Component identifier \"po1\" is defined twice")]
    fn test_redefined() {
        let cs = ComponentStore {
            store: vec![Rc::new(ProbeOut::new("po1")), Rc::new(ProbeOut::new("po1"))],
        };

        let simulator = Simulator::new(cs).unwrap();

        assert_eq!(simulator.cycle, 1);
    }

    #[test]
    fn test_get_input_val() {
        let cs = ComponentStore {
            store: vec![Rc::new(ProbeOut::new("po1"))],
        };

        let simulator = Simulator::new(cs).unwrap();

        assert_eq!(simulator.cycle, 1);
        let _ = simulator.get_input_value(&Input::new("po1", "out"));
    }

    #[test]
    #[should_panic(expected = "Component \"po1\", field \"missing\" not found.")]
    fn test_get_input_out_of_range() {
        let cs = ComponentStore {
            store: vec![Rc::new(ProbeOut::new("po1"))],
        };

        let simulator = Simulator::new(cs).unwrap();

        assert_eq!(simulator.cycle, 1);
        let _ = simulator.get_input_value(&Input::new("po1", "missing"));
    }

    #[test]
    fn test_get_input_fmt() {
        let cs = ComponentStore {
            store: vec![Rc::new(Constant::new("c", (0.0, 0.0), 0))],
        };

        let simulator = Simulator::new(cs).unwrap();

        assert_eq!(simulator.cycle, 1);
        let _ = simulator.get_input_fmt(&Input::new("c", "out"));
    }
}
