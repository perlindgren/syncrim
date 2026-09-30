// Shared helpers for testing clock/un_clock of components inside a real Simulator.
//
// Tests should go through Simulator::clock and Simulator::un_clock rather than calling
// the component directly, since un_clock depends on how the simulator orders things:
// sim.cycle is incremented after all components are clocked, but is not decremented
// until after all components are unclocked, and un_clock sees the signal values as
// they were at the end of the clock being undone.
use serde::{Deserialize, Serialize};
use std::{any::Any, fmt::Debug, rc::Rc};
use syncrim::{
    common::{
        Component, ComponentStore, Condition, Id, OutputType, Ports, RunningState, Simulator,
    },
    signal::SignalValue,
};

pub const DRIVER_ID: &str = "drv";

/// Drives its outputs from a script during clock, like the rest of a circuit would.
/// ProbeOut can't be used, since un_clock needs the inputs as they were during that cycle
#[derive(Serialize, Deserialize)]
pub struct TestDriver {
    // (port, value when not scripted)
    #[serde(skip)]
    pub outputs: Vec<(&'static str, SignalValue)>,
    // (cycle, port, value)
    #[serde(skip)]
    pub script: Vec<(usize, &'static str, SignalValue)>,
}

#[typetag::serde]
impl Component for TestDriver {
    fn to_(&self) {}
    fn get_id_ports(&self) -> (Id, Ports) {
        (
            DRIVER_ID.into(),
            Ports::new(
                vec![],
                OutputType::Combinatorial,
                self.outputs.iter().map(|(port, _)| *port).collect(),
            ),
        )
    }
    fn clock(&self, sim: &mut Simulator) -> Result<(), Condition> {
        for (port, idle) in &self.outputs {
            let value = self
                .script
                .iter()
                .find(|(cycle, p, _)| *cycle == sim.cycle && p == port)
                .map_or(*idle, |(_, _, v)| *v);
            sim.set_out_value(DRIVER_ID, port, value);
        }
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(feature = "gui-egui")]
#[typetag::serde]
impl syncrim::common::EguiComponent for TestDriver {}

#[cfg(feature = "gui-egui")]
pub type TestComponent = dyn syncrim::common::EguiComponent;
#[cfg(not(feature = "gui-egui"))]
pub type TestComponent = dyn Component;

/// Build a simulator from a driver and the components under test.
/// Simulator::new clocks once, so scripts start at cycle 1
pub fn test_sim(driver: TestDriver, components: Vec<Rc<TestComponent>>) -> Simulator {
    let mut store: Vec<Rc<TestComponent>> = vec![Rc::new(driver)];
    store.extend(components);
    let sim = Simulator::new(ComponentStore { store }).unwrap();
    assert_eq!(sim.cycle, 1);
    sim
}

pub fn get_component<T: 'static>(sim: &Simulator) -> &T {
    sim.ordered_components
        .iter()
        .find_map(|c| c.as_any().downcast_ref())
        .unwrap()
}

/// Clock `cycles` times, then unclock all the way back checking that every cycle is restored,
/// then clock forward again checking that the replay gives the same result.
/// The replay catches state that un_clock forgot to restore, but that only shows up later.
pub fn assert_round_trip<S: PartialEq + Debug>(
    mut sim: Simulator,
    cycles: usize,
    snapshot: impl Fn(&Simulator) -> S,
) {
    let mut snapshots = vec![snapshot(&sim)];
    for _ in 0..cycles {
        // ignore errors, such as writing invalid data
        sim.running_state = RunningState::Stopped;
        sim.clock();
        snapshots.push(snapshot(&sim));
    }
    for cycle in (0..cycles).rev() {
        sim.un_clock();
        assert_eq!(
            snapshot(&sim),
            snapshots[cycle],
            "after unclock to step {cycle}"
        );
    }
    for (cycle, expected) in snapshots.iter().enumerate().skip(1) {
        sim.running_state = RunningState::Stopped;
        sim.clock();
        assert_eq!(&snapshot(&sim), expected, "after replay to step {cycle}");
    }
}
