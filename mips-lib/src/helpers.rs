use syncrim::common::Simulator;

pub fn find_component_with_type<'a, T: 'static>(sim: &'a Simulator, id: &str) -> Option<&'a T> {
    let v = &sim.ordered_components;
    let o_comp = v.iter().find(|x| x.get_id_ports().0 == id);
    if let Some(comp) = o_comp {
        // deref to get &dyn EguiComponent
        let comp_any = (*comp).as_any();
        comp_any.downcast_ref()
    } else {
        None
    }
}
