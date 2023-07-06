# SyncRim Build

In this document the design of feature handling, conditional compilation and test is described in further detail.

The design goal is to make `SyncRim` as frontend (GUI) independent as possible. To that end the we make use of (cargo) features.

The (root) `SyncRim` `Cargo.toml` looks something like this:

```toml
...
[features]
default = ["gui-vizia"]
gui-vizia = ["vizia"]
gui-egui = []
...
```

This implies when compiled or used as a library `gui-vizia` will de enabled by default. (Currently we pull `vizia` from git, but later when released on crates we will like use the official release.)

The `gui-egui` feature is currently a placeholder for an alternative `egui` based frontend. Wheres both `vizia` and `egui` brings in a lot of dependencies (several hundred under Linux), we don't want to carry the extra weight of both being pulled in at the same time. Without changing the `Cargo.toml` file you may disable and/or change front-end, e.g.:

```shell
cargo run --example add --no-default-features
```

In this case you just get a dump of the `.json` representation of the `add` example.

So let's take a closer look at the `add` example to see what happened:

```rust
    ...
    let path = PathBuf::from("add.json");
    cs.save_file(&path);

    #[cfg(feature = "gui-vizia")] # <---
    syncrim::gui_vizia::gui(&cs, &path);
}
```

The `#[cfg(feature = "gui-vizia")]` applies to the following item (`syncrim::gui_vizia::gui(&cs, &path);`), if the set of features enabled does not include `gui-vizia` the the item is excluded. That is done very early in the compilation process, thus even if `syncrim::gui_vizia` does not exist in scope we won't get an error.

Now, let's take a look at the `SyncSim` top level `lib.rs`:

```rust
...
// Vizia frontend
#[cfg(feature = "gui-vizia")] # <---
pub mod gui_vizia;
...
```

In this case `pub mod gui_vizia` will be excluded from the build. That implies that whatever is in the `gui_vizia` folder is never compiled, and whatever dependencies the code within this folder requires are never checked.

This allows us to optionally pull in the (expensive) `vizia` dependency in the `Cargo.toml`:

```toml
...
[dependencies.vizia] # <--- 2) check `vizia` feature
git = "https://github.com/vizia/vizia.git"
optional = true # <---

[features]
default = ["gui-vizia"]
gui-vizia = ["vizia"] # --> 1) set `vizia` feature
gui-egui = []
...
```

The `optional = true` implies that the dependency is brought in only if the `vizia` feature is set. In this way `features` can be elegantly used to control optional dependencies.

Now we are talking!

```shell
cargo run --example add
```

Will build the example with the `gui-vizia` feature set, thus:

- will bring in `vizia` crate,

- compile compile the `gui-vizia` folder where the GUI resides,

- will compile `syncrim::gui_vizia::gui(&cs, &path);`, and consecutively

- start the GUI.

On the other hand:

```shell
cargo run --example add --no-default-features
```

Will build the example without any feature set, thus:

- will _not_ bring in the `vizia` crate,

- will _not_ compile the `gui-vizia` folder,

- will _not_ compile the call `syncrim::gui_vizia::gui(&cs, &path);`, and consecutively

- will _not_ start the GUI.

You can check the number of dependencies brought in both cases:

```shell
cargo clean
cargo run --example add --no-default-features
cargo clean
cargo run --example add
```

The difference in huge (some 160 crates vs. some 430 crates under Arch Linux X11).

---

## Cargo test

Automatic testing of GUI code is fundamentally hard, thus for now we focus on testing the logic and stateful behavior of our components.

Assuming our simulator is correctly implemented we can use it to drive the test. So essentially, we are doing an _integration test_ on component(s) and the simulator.

In Rust integration tests are found in the `tests` folder, we have `component_tests.rs`:

```rust
    ...
    let cs = ComponentStore {
        store: vec![
            Rc::new(ProbeOut::new("po1")),
            Rc::new(ProbeOut::new("po2")),
            Rc::new(Add {
                id: "add".to_string(),
                pos: (0.0, 0.0),
                a_in: Input::new("po1", 0),
                b_in: Input::new("po2", 0),
            }),
        ],
    };
    ...
```

Here, we define a simulation "model" for the `Add` component. The `Add` component instance ("add") refers to the two `ProbeOut` components ("po1" and "po2").

```rust
    ...
    let mut clock = 0;
    let mut simulator = Simulator::new(&cs, &mut clock);

    assert_eq!(clock, 1);

    // outputs
    let add_val = &Input::new("add", 0);
    let add_overflow = &Input::new("add", 1);

    // reset
    assert_eq!(simulator.get_input_val(add_val), 0 + 0);
    assert_eq!(simulator.get_input_val(add_overflow), false as Signal);

    ...
```

Here we instantiate the simulator for the "model", and check (assert) that the `clock` is set to 1. We define hooks for the different outputs of the "add" instance, and check (assert) that their initial state is correct. (All signals are set to 0 on reset so the `add_val` should be 0 + 0, and `add_overflow` should be false).

So far so good, now let's try a more interesting addition:

```rust
    simulator.set_id_index("po1", 0, 42);
    simulator.set_id_index("po2", 0, 1337);

    simulator.clock(&mut clock);

    assert_eq!(clock, 2);
    assert_eq!(simulator.get_input_val(add_val), 42 + 1337);
    assert_eq!(simulator.get_input_val(add_overflow), false as Signal);
    ...
```

Here we set the inputs to the adder to be 42 and 1337 accordingly, we call `simulator.clock` to perform one simulation step, and finally check that the results are correct, 42 + 1337 without any overflow.

The Rust testing framework is quite comprehensive and well documented elsewhere (e.g. [The Rust Book](https://doc.rust-lang.org/book/ch11-03-test-organization.html?highlight=test#integration-tests)), so we cut to the chase:

```shell
cargo test
```

This runs a number of tests, including the integration test. At this point all tests should pass.

We can now willingly make it fail (for educational purpose). Change the ` simulator.set_id_index("po1", 0, 42);` to `simulator.set_id_index("po1", 0, 666);` and re-run the command. (Either from terminal or clicking on the test button above `fn test_add()...`).

As expected the test fails:

```shell
...
thread 'test_add' panicked at 'assertion failed: `(left == right)`
  left: `2003`,
 right: `1379`', tests/component_tests.rs:42:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
...
```

The test also checks the case of positive overflow. As an exercise you may write your own test for negative overflow.

---

## Cargo test --no-default-features

As we now already know we can control compilation and execution by cargo features. Compare the two:

```shell
cargo clean
cargo test --no-default-features
cargo clean
cargo test
```

Also here you should observe that `--no-default-features` reduces the number of brought in crates, thus useful to reduce CI testing.

## Cargo workspace

`SyncRim` is designed as a library for building simulators. As an example you find the `mips` crate (workspace member) providing components specific to `mips`, and (in the future) a set of `mips` models, like single-cycle, pipelined, and CP0 co-processor extensions.

To test the `mips` workspace member:

```shell
cargo test --package mips
```

To test the complete workspace (`SyncRim` and all workspace members):

```shell
cargo test --workspace
```

Also here you can add `--no-default-features` to reduce the dependencies and thus testing time.

---

## Workspace members

Each workspace member is compiled (and tested) as a crate in its own right, where features apply to the crate at hand. So let's have a look at the `mips` `Cargo.toml`:

```toml
[dependencies]
serde = "1.0.166"
serde_derive = "1.0.166"
typetag = "0.2.8"
serde_json = "1.0.96"

[dependencies.syncrim]
path = "../"
default-features = false

[features]
default = ["gui-vizia"]
gui-vizia = ["syncrim/gui-vizia"]
gui-egui = ["syncrim/gui-egui"]
```

Here we define a set of features (`gui-vizia`, `gui-egui`)and their propagation. (For convenience we use the same names as in the root crate, but it is not required.)

Both features are propagated to the `syncrim` (root) crate. Features in Rust are additive (you can include a feature but not exclude a feature). By setting `default-features = false` on the `syncrim` crate we start from a clean slate (independent on `default` features set in the `syncrim` (root) crate).

As the `mips` crate has `default = ["gui-vizia"]` this means that both the `mips` and `syncrim` crates will be compiled with `gui-vizia` set, whereas `--no-default-features` will compile both crates without any features set (in this case overriding the `default = ["gui-vizia]` set in the `syncrim` crate).

Now, take a breather - read this section again, and make sure you understand the features interplay between these two crates.

### Re-export

Hmm, so the `mips` crate internally contains GUI views for added components but does not by itself declare any dependency to `vizia`. How can that work?

First take a look at the `lib.rs` in the top level `syncrim` crate (root).

```rust
...
#[cfg(feature = "gui-vizia")]
pub use vizia;
```

This implies that if `syncrim` was compiled with `gui-vizia` set, then the `vizia` crate will be re-exported (and accessible as `syncrim::vizia`).

Now look at the `mips` `lib.rs`:

```rust
// Vizia frontend
#[cfg(feature = "gui-vizia")]
pub mod gui_vizia;            # <--- syncrim export

// Re-export
#[cfg(feature = "gui-vizia")]
pub mod gui_vizia;            # <--- vizia crate re-export
```

Similarly to the `syncrim` crate we build the `mips` `gui_vizia` module only in case `gui-vizia` is set, thus code inside the `gui_vizia` folder can safely access both `gui_vizia` folder of the `syncrim` crate and the `syncrim::vizia` external crate.

As an example, the `reg_file.rs`:

```rust
use crate::components::RegFile;
use syncrim::{
    common::ViziaComponent,
    gui_vizia::tooltip::new_component_tooltip, # <-- syncrim export
    vizia::{                                   # <-- vizia crate re-export
        prelude::*,
        vg::{Color, Paint, Path},
    },
};

#[typetag::serde]
impl ViziaComponent for RegFile {
    ...
```

---

## Selecting frontend (GUI)

By default the `vizia` frontend will be selected. If you want to select another frontend:

```shell
cargo run --example <model> --no-default-features --features gui-egui
```

Notice, you need to pass both `--no-default-features` (to disable `vizia`) and `--features gui-egui` to select EGui.

## VSCode integration

The `.vscode/settings.json` control the workspace build options, e.g. to run with EGui:

```json
    ...
    "rust-analyzer.cargo.noDefaultFeatures": true,  # examples will build/run without any gui selected
    "rust-analyzer.cargo.features" : ["gui-egui"],  # examples will build/run with egui (notice you need) both .noDefaultFeatures and .features 
    ...
```

## Summary

In this document we covered features, feature propagation, test and workspace member integration for the `SyncRim` project. This is of course not the only possible solution and breaking changes might be introduced.
