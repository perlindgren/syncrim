use std::rc::Rc;

use crate::common::{ComponentStore, EguiComponent, Input};
use crate::components::Wire;

pub fn autowire(mut cs: ComponentStore) -> ComponentStore {
    let mut wires: Vec<Rc<dyn EguiComponent>> = vec![];

    // for each component
    for destination_component in &cs.store {
        let dest_comp_id = destination_component.get_id_ports().0;
        // for each port in destination component
        for input_port in destination_component.get_id_ports().1.inputs.iter() {
            let source_port = &input_port.input;
            let dest_comp_field = &input_port.port_id;

            // find component with correct source id
            let source_component = cs
                .store
                .iter()
                .filter(|comp| comp.get_id_ports().0 == source_port.id) // compare id
                .next()
                .expect(&format!("can't find comonent with id {}", source_port.id));

            // create wire with correct source destination and positions

            let s_id = &source_port.id;
            let s_field = &source_port.field;
            let d_id = &dest_comp_id;
            let d_field = &dest_comp_field;

            wires.push(Wire::rc_new(
                &format!("from {}:{} to {}:{}", s_id, s_field, d_id, d_field),
                vec![
                    source_component
                        .get_port_location(Input::new(s_id, s_field))
                        .unwrap_or(source_component.get_pos()),
                    destination_component
                        .get_port_location(Input::new(s_id, s_field))
                        .unwrap_or(destination_component.get_pos()),
                ],
                source_port.clone(),
            ))
        }
    }
    cs.store.append(&mut wires);
    return cs;
}
