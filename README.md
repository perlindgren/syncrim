# SyncSim

Main simulator and generic components.

---

## Types for the storage model

The model provides:

```rust
pub struct Ports {
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
}
```

Where:

```rust
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Input {
    pub id: String,
    pub index: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Output {
    Constant(u32),
    Combinatorial,
    Sequential,
}
```

---

## Traits

```rust
#[typetag::serde()]
pub trait Component {
    // placeholder
    fn to_(&self) {}
    // returns the (id, Ports) of the component
    fn get_id_ports(&self) -> (String, Ports);
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
#[derive(Serialize, Deserialize)]
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
pub struct LensValues {
    pub values: Vec<u32>,
}
```

The `SimState` holds the values and the mapping between identifiers and ports.

```rust
#[derive(Debug)]
pub struct SimState {
    pub lens_values: LensValues,
    pub id_ports: IdPorts,
}
```

The initial simulator state is constructed from a `ComponentStore`.

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
