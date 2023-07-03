# SyncRim

A graphical simulator for synchronous circuits written in Rust based on the [vizia](https://github.com/vizia/vizia) framework. The long term goal is to simulate models of modern embedded processors, giving the user control over - and insight in - the inner workings of a CPU without the need for traditional RTL waveform simulations which are hard to interpret. Other use cases stretch exploratory research of synchronous circuits, hardware software co-design etc.

`SyncRim` is heavily inspired by an in-house [SyncSim](https://syncsim.sourceforge.net/) development at Luleå University of Technology. SyncSim has been successfully used in teaching Micro-computer Engineering at LTU for almost two decades, but it starts to show its age. `SyncRim` a Rust implementation of `SyncSim` is tempting :)

## Dependencies

For faster builds under Linux, we depend on `clang` and `mold` being installed. You may disable the alternate linking in `.cargo/config.toml`, if you want to stick with `lld` comment out the linker configuration.

To test `SyncRim` run:

```shell
cargo run --example <example>
```

This will build and run the corresponding example and as a side effect create a loadable model (`<example>.json`).

To load and run the created model (`<example>.json`).

```shell
cargo run -- -model <example>.json
```

Alternatively, you can run the `mips` example from the `mips` folder.

```shell
cd mips
cargo run --example mips
```

And consequently run the created model (`mips.json`).

```shell
cd mips # if not already done
cargo run
```

You can also run the examples correspondingly in `vscode`.

After the initial models have been generated you may alter them (edit the `json` files and just run the corresponding `main` to simulate the altered model).

Disclaimer: you will run into panics in case your model is faulty, sorry no nice error messages to be expected. Circular dependent combinatorial circuits are considered illegal (for good reasons). Direct register to register dependencies (without intermittent combinatorial components) will likely render undefined behavior.

---

## `winit` and Scaling

At least under Linux, scaling can be incorrectly determined by `winit`. This will cause various graphical artifacts (e.g., weird clipping of fonts in tooltips). It is not problem with `Vizia` or any other GUI per-se, but a problem of Linux not defining a stable way of providing the current scaling. A workaround is possible by setting the environment variable `WINIT_X11_SCALE_FACTOR` to `1.0` (or any other scaling wanted). An example for `fish` shell:

```fish
set -x WINIT_X11_SCALE_FACTOR 1.0
```

---

## Key design goals

- `SyncRim` should be modular (providing a user extendable library of components and a simulation engine).

- Models in `SyncRim` should be possible to alter and load without the need to re-compile the application.

## Technologies used

- [vizia](https://github.com/vizia/vizia) was chosen based on numerous criteria:

  - Rust based from the ground up, offering good ergonomics.

  - Cross platform (Linux/Windows/OSX).

  - Modern declarative approach to GUI design, allowing scaling and CSS based theming.

  - Great community support by `geom3trik`, other users and co-developers.

- [serde](https://github.com/serde-rs/serde) for serialization/deserialization (storing/loading) models.

- [typetag](https://github.com/dtolnay/typetag) to serialize/deserialize trait objects (component instances).

- [petgraph](https://github.com/petgraph/petgraph) for underlying graph handling.

- [graphviz](https://graphviz.org/) for visualizing the model dependencies.

## Design overview

`SyncRim` is based on the following guiding principles:

- A clear separation between synchronous and combinatorial components.

- A topological order is established from the data dependencies between components.

- Next state computation amounts to traversing the ordered component graph. (Ensures that each component is evaluated exactly once.).

- A clear separation between Models and Views:

  - Models are used to handle events in the system (business logic). The top level model has access to the global simulation state.

  - Views are used for graphical representation, with immutable access to global state (through Vizia `Lens` abstraction).

- Flat hierarchy (no sub-components, at least for now). However the graphical representation may contain sub-views.

- Grid based component layout. (However sub-views may use automated layout).

Modularity:

- `SyncRim` is a library providing a set of commodity components.
- Additional components may be defined in re-usable libraries.

- A `SyncRim` application can be compiled as a stand-alone application supporting components from various libraries.

- A compiled `SyncRim` application can load/run models for the supported set of components without re-compilation. E.g, a

  - `SyncRim-MIPS` application imports the `SyncRim` base and implements (or imports) additional components needed for defining a simulation model for the `MIPS` architecture, while a
  - `SyncRim-RISC-V` application imports the `SyncRim` base, and similarly implements (or imports) additional architecture specific components for the `RISC-V` architecture.

  In this way, common components can re-used between targets, while the `SyncRim` base can be kept small and maintainable.

---

## POC implementation

`SyncRim` is in early development. The POC implementation currently demonstrates:

- The `Component` interface (trait) for defining components.

- Storing and loading simulation models using `serde`.

- Establishing topological order for component models.
- Simulation by ordered traversal and simulation state mutation.

- Graphical representation of simulation state.

  - `Clock` for progressing state.
  - `UnClock` for reverting state.
  - `Reset` to set initial state.
  - `Play` to enter continuous clocking moe.
  - `Pause` to exit continuous clocking mode.

  Notice, the system is initially in `Reset`. (Continuous mode not yet implemented.)

- Menu and keyboard shortcuts (backing functionality mostly unimplemented.)
- Limited set of commodity components:

  - `Add` a two input adder

  - `Mux` a generic multiplexer, with a select 0..N-1, and N input signals.

  - `Constant` a constant value.

  - `Probe` (a component for visualization of simulation signal)

  - `Register` a component storing a signal value and propagating inputs to outputs on clock.

- Modularization:

  - A `MIPS` component extension holding
    - `InstrMem` component (very primitive, just a POC)
    - `RegFile` component (should be usable)

- Graphviz support:
  On startup the initial model will be serialized as a `<model>.gv` file. This can be previewed directly in `vscode` using the [dot](https://marketplace.visualstudio.com/items?itemName=tintinweb.graphviz-interactive-preview) plugin and/or by the `dot` command line tool (typically packed together with a `graphviz` distribution).

  E.g.

  ```shell
  dot -Tpdf <model>.gv -o <model>.pdf
  ```

## TODO

- The GUI is currently very primitive (only a transport providing `Clock`, `UnClock`, `Reset`, `Play`, `Pause` controls). Envisioned functionality:

  - Run for given number of cycles. Run until signal condition. Restart. Step backwards. Step back until signal condition. Reset. Etc.

  - A model editor.

  For now models are exported in `json` format offering relatively easy editing but a graphical editor is of course better.

  - `Re-load` (CTRL R), or
  - `Open` (CTRL O) of altered `json` is supported.

- The simulator state is current consisting of `Vec<Signal>` where each signal amounts to a `u32` value. Here we can think of some sort of bit-vector representation.

- Topological order may be incorrect in case of register to register dependencies without intermittent combinatorial components.

- The set of commodity components should be extended.

  - Generic instruction/data memory components, with integration to signal condition monitoring (allow e.g., breakpoints in the code, or addresses/data reads/writes).

  - Generic de-multiplexer, sign/zero extend components etc.

- Extended set of architecture dependent components:

  - Instruction decoders, ALUs, etc.

  Ultimately the generic and architecture dependent components should be sufficient to model common embedded processors.

- Use of logging framework. Currently, neither `SyncRim` nor `Vizia` uses any logging framework, however `Vizia` provides a `log!` macro for transparent tracing on `Wasm` and other platforms. This is currently not used by `SyncRim`).

- Error handling: Currently, `SyncRim` does not implement any graceful error handling (will abort with a panic).

- Testing: CI based unit and integration tests. Currently there are no tests at all.

- Probably a zillion other things.

---

## Implementation specifics

In the following we highlight some specifics of the current design.

## Types for the storage model

The `common` module provides:

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

These types are used to build components.

---

## Traits

```rust
pub trait Component {
    // placeholder
    fn to_(&self) {}

    // returns the (id, Ports) of the component
    fn get_id_ports(&self) -> (String, Ports);

    // evaluation function
    fn evaluate(&self, _simulator: &mut Simulator) {}

     // create view
    fn view(&self, _cx: &mut Context) {}
}
```

Any component must implement the `Component` trait, (`evaluate` and `view` are optional).

For serialization to work, `typetag` is derived for the `Component` trait definition as well as its implementations. Under the hood, the `dyn Trait`s are handled as enums by serde.

---

## Components

SyncSim provides a set of predefined components:

- `Constant`
- `Register`
- `Mux`
- `Add`
- `Probe`

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

In order to view the simulator state we store (current) values as a `Vizia` `Lens`.

```rust
#[derive(Lens, Debug, Clone)]
pub struct Simulator {
    ...
    pub sim_state: Vec<Signal>,
}
```

The `Simulator` holds the values and the mapping between identifiers and ports.

```rust
pub struct Simulator<'a> {
    pub id_start_index: IdStartIndex,

    // Components stored in topological evaluation order
    pub ordered_components: Components,
}
```

## Simulator Implementation

The initial simulator state is constructed from a `ComponentStore`.

```rust
impl Simulator {
    pub fn new(component_store: &ComponentStore) -> Self
    ...

```

The `Simulator` holds the evaluation order of components in `ordered_components`, and the mutable state (`sim_state`).

To progress simulation, we iterated over the `ordered_components`:

```rust
impl Simulator {
    ...
    // iterate over the evaluators
    pub fn clock(&mut self, clock: &mut Clock) {
        for component in &self.ordered_components {
            component.evaluate(self, sim_state);
        }
    }
}
```

---

## Example component `Add`

The `Add` component is defined by:

```rust
#[derive(Serialize, Deserialize)]
pub struct Add {
    pub id: String,
    pub pos: (f32, f32),
    pub a_in: Input,
    pub b_in: Input,
}
```

An instance of `Add` might look like this:

```rust
Add {
    id: "add1",
    pos: (200.0, 120.0),
    a_in: Input {
      id: "c1",
      index : 0
    },
    b_in: Input {
      id: "r1",
      index: 0
    }
}
```

The corresponding serialized `json` looks like this:

```json
        {
            "type": "Add",
            "id": "add1",
            "pos": [
                200.0,
                120.0
            ],
            "a_in": {
                "id": "c1",
                "index": 0
            },
            "b_in": {
                "id": "r1",
                "index": 0
            }
        },
```

The `Add` component implements `get_id_ports`, `evaluate` and `view`. The first is used on loading a model for determining the dependencies (and from that the topological order), the second is used for simulation and the third to create a `Vizia` view of the component.

Notice that the `get_id_ports` returns a vector of output types. In this case the component has just one output (the sum of inputs computed as a function). On loading the model, consecutive space is allocated for each output and a mapping created from the component identifier to the allocated space.

`evaluate` retrieves the input values from the simulator, computes the sum and stores it at the first position of the allocated space. In case a component has several outputs, the offset is passed, e.g., `simulator.set_id_index(..., 1, ...)`, to set the 2nd output of the component.

```rust
impl Component for Add {

    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports {
                inputs: vec![self.a_in.clone(), self.b_in.clone()],
                out_type: OutputType::Combinatorial,
                outputs: vec![Output::Function],
            },
        )
    }

    // propagate addition to output
    fn evaluate(&self, simulator: &mut Simulator) {
        // get input values
        let a_in = simulator.get_input_val(&self.a_in);
        let b_in = simulator.get_input_val(&self.b_in);

        // compute addition (notice will panic on overflow)
        let value = a_in + b_in;

        // set output #0
        simulator.set_id_index(&self.id, 0, value);
    }

    // create view
    fn view(&self, cx: &mut Context) {

        View::build(AddView {}, cx, |cx| {
            Label::new(cx, "+")
                .left(Percentage(50.0))
                .top(Pixels(40.0 - 10.0));
        })
        .position_type(PositionType::SelfDirected)
        .left(Pixels(self.pos.0 - 20.0))
        .top(Pixels(self.pos.1 - 40.0))
        .width(Pixels(40.0))
        .height(Pixels(80.0))
        .tooltip(|cx| new_component_tooltip(cx, self));
    }
}
```

The `Add` component is anchored at `pos` with height 80 and width 40 pixels. The tooltip is used to show the inputs and outputs on hovering.

---

## Development

### VsCode

Recommended plugins:

- `rust analyzer`
- `Code Spell Checker`, to keep code and comments nice and clean.

Settings:

It can be convenient to use `json` formatter tied to format on save, this way we can keep models in easy readable and editable shape.

### Clippy

Before merging, make sure that it passes clippy, if not fix or locally allow the clippy warning. This allows us to later review why clippy warned us, in order to stay more (or less) idiomatic.

---

## License

TDB (likely MIT + Apache)
