# SyncSim

Main simulator and generic components.

---

## Types for the storage model

The model provides:

```rust
pub struct Ports {
    pub inputs: Vec<Input>,
    pub out_type: OutputType,
    pub outputs: Vec<Output>,
}
```

Where:

```rust
pub struct Input {
    pub id: String,
    pub index: usize,
}

pub enum OutputType {
    // Will be evaluated as a combinatorial function from inputs to outputs
    Combinatorial,
    // Will be evaluated as synchronous from input to output
    Sequential,
}

pub enum Output {
    // Will be evaluated as a constant (function without inputs)
    Constant(u32),
    // Will be evaluated as a function
    Function,
}
```

---

## Traits

```rust
pub trait Component {
    // placeholder
    fn to_(&self) {}

    // returns the (id, Ports) of the component
    fn get_id_ports(&self) -> (String, Ports);

    // evaluation function
    fn evaluate(&self, _simulator: &Simulator, _sim_state: &mut SimState) {}
}
```

For serialization to work, `typetag` is derived for the `Component` trait definition as well as its implementations. Under the hood, the dyn Traits are handled as enums by serde.

---

## Components

SyncSim provides a set of predefined components:

- `Constant`
- `Register`
- `Mux`
- `Add`

The components implement the `Component` trait, used to build a various mappings.

A (simulation) model can extend the set of components (see the `mips` member crate).

A model is defined by the storage:

```rust
type Components = Vec<Rc<dyn Component>>;

#[derive(Serialize, Deserialize)]
pub struct ComponentStore {
    pub store: Components,
}
```

---

## Simulator State

In order to view the simulator state we store (current) values as `vizia` `LensValues` (deriving the `Lens` trait).

```rust
#[derive(Lens, Debug, Clone)]
pub struct SimState {
    pub lens_values: Vec<u32>,
}
```

The `SimState` holds the values and the mapping between identifiers and ports.

```rust
pub struct SimState {
    pub lens_values: LensValues,
}

pub struct Simulator<'a> {
    pub id_start_index: IdStartIndex,

    // Components stored in topological evaluation order
    pub ordered_components: Components,
}
```

## Simulator Implementation

The initial simulator state is constructed from a `ComponentStore`.

```rust
impl<'a> Simulator<'a> {
    pub fn new(component_store: &'a ComponentStore) -> (Self, SimState)
    ...

```

The `Simulator` is immutable (holding the evaluation order of components), while the `SimState` holds the mutable state (in terms of lensed values).

To progress simulation, we iterated over the `ordered_components`:

```rust
impl<'a> Simulator<'a> {
    ...
    // iterate over the evaluators
    pub fn clock(&self, sim_state: &mut SimState) {
        for component in &self.ordered_components {
            component.evaluate(self, sim_state);
        }
    }
}
```

As an example the `add` component implements `evaluate` by reading the two inputs, adding them, and storing the value (to output position 0). Todo: We might want to structure the outputs similar to the inputs.

```rust
impl Component for Add {
    ...
    // propagate addition to output
    fn evaluate(&self, simulator: &Simulator, sim_state: &mut SimState) {
        // get input values
        let a_in = simulator.get_input_val(sim_state, &self.a_in);
        let b_in = simulator.get_input_val(sim_state, &self.b_in);

        // compute addition (notice will panic on overflow)
        let value = a_in + b_in;

        // set output
        simulator.set_id_index(sim_state, &self.id, 0, value);
    }
}
```

---

## Development

### VsCode

Recommended plugins:

- `rust analyzer`
- `Code Spell Checker`, to keep code and comments nice and clean.

Settings:

It can be convenient to use json formatter tied to format on save, this way we can keep models in easy readable and editable shape.

### Clippy

Before merging, make sure that it passes clippy, if not fix or locally allow the clippy warning. This allows us to later review why clippy warned us, in order to stay more (or less) idiomatic.

---

## License

TDB (likely MIT + Apache)
