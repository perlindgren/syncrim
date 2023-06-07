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
pub struct ComponentStore {
    pub store: Vec<Box<dyn Component>>,
}
```

For serialization to work, `typetag` is derived for the `Component` trait definition as well as its implementations. Under the hood, the dyn Traits are handled as enums by serde.

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
    pub ordered_components: Vec<&'a Box<dyn Component>>,
}
```

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


---

## Simulator Implementation

This allows us to get and set individual lensed values:

```rust
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
```

---

## Development

### VsCode

Recommended plugins:

- `rust analyzer`
- `Code Spell Checker`, to keep code and comments nice and clean.

Settings:

It can be convenient to use json formatter tied to format on save, this way we can keep models in easy readable and editable shape.

---

## License

TDB (likely MIT + Apache)




