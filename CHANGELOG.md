# Changelog

Tracking changes per date:

## 240911
- Added so that simulator now tracks what component condition
- Added simulator running state, such as running, halt or error
- Added stepping functionality
- Added functionality to gui to show running state and component condition

## 240909

- Added un-clock history for set of active components

## 240908

- Added functionality to determine set of active components

## 230801

- Simulator run/halt is implemented in `vizia` using a simple eventing mechanism. Later we might want to spawn a simulation thread for faster execution (right now its tied to frame rate).

## 230731

- Return type from `clock` (`fn clock(&self, _simulator: &mut Simulator) -> Result<(), Condition`).

- RISC-V cross compilation.

## 230727

- `Signal` type now incorporates formatting. This allows the default formatting to be set on a signal on creation. The data and formatting can be read/written separately by setters/getters.
  
- Internal component fields are now `pub(crate)`. This allows internal component structure to be hidden outside the crate, thus examples and other users cannot affect the component state, also we are free to change internal repr without affecting examples/users (given that the API can remain stable).

- `rc_new` implemented for all components. (Examples updated.) We might want to change `new` to `_new` and `rc_new` to `new`.
  
## 230725

- Added RISC-V components and model

- Implemented the `ProbeAssert` component, that assert a set sequence of inputs. Made some updates so reading outside of the assert/stim buffers gives `Signal::Unknown` instead of panic (if not in test mode).

  Asserts are run only in test mode, allowing gui testing to be more robust.

- Refactored, `clock` as `cycle` and put it in the `Simulator` (thanks to Fredrik for suggesting this a while back). Now the Simulator holds the complete state, which is better.

- Implemented the `ProbeStim` component, to provide a set sequence of outputs.
  
## 230721

- Added rudimentary support for a structured `Signal` type, inspired by HDLs.

## 230719

- `ProbeEdit`, a component for interactive debugging (and maybe some end usage as well). It allows you to enter a value (dec/hex) for a signal. It acts as a register so its content will be used in the next clock cycle (one could also think of changing this to act directly by triggering some re-evaluation, not sure). `ProbeEdit` also implements a proper history buffer so you can reverse the simulation.

- `un_clock` method in the `Component` trait. It does not alter the simulation state (for that we already have a history buffer), components with internal state (e.g., the `ProbeEdit` component) implements `un_clock` and keep a local history. Maybe we will have some helper functions to that end.

## 230718

- Panes, right click component to display component interior on left panel. Close left panel view by the X (Close) button.

- Left panel views can be folded/unfolded

## 230717

- mips/reg_file update, showcasing interior mutability and gui_vizia poc.
  
## 230714

- `fern` based logger.

- Fixed clippy lint for Rust 1.71

- WIP support for named outputs.

- Added caching to the Github workflow to improve job times.

## 230713

- Added bounds checking for `Simulator` (`set_id_index`, `get_input_val`), panics on out of bounds.
- Added re-definition check for `Simulator`, panics on re-definition.
- Added units tests to the above.

## 230705

- Refactoring of `Component` trait such that logic part is separated from GUI part.

- Test framework added, where integration (simulation tests are found in the `tests` folders.)

## 230703

- Webhooks for Discord channel.

- `.gitignore` to exclude `.gv` and `.json` files.

- `SignExtension` component.

Compilation times might be annoying. It seems that under Linux there is a lot more dependencies than under Windows, so building `SyncRim` is faster on Win10 than under Linux (stock settings). There are a number of things to try out to reduce compilation times (and in particular `hot` iterations).

- Disable generation of debug (DWARF) info. `debug = false` (or `debug = 0`). If stack backtraces are needed use `debug = 1`, which is still faster than `debug = 2` (`debug = true` equivalent).
- Use `mold` instead of `lld`.

Together `debug = 0` and `mold` linking may yield significant improvements. (On the `7950x3d`, `hot` re-compiles improved from around 4 seconds to 0.3 seconds (in the range of measuring errors for vscode interactions etc.) With `debug = true` + `mold` 0.5 seconds or so, both acceptable, `mold` makes the biggest difference to incremental builds.

See `Cargo.toml` and `.cargo/config.toml` for updated configuration. Notice, you need to install `mold` through your package manager for this to work.

## 230702

- `path` no longer stored in `.json` model.
- `path` passed as `&PathBuf` to `gui` and `component_store`.
- `path` stored as `PathBuf` in the `gui`.

Finally, SyncRim can now open models through `Open` (CTRL-O) in `gui` (`ReOpen` (Ctrl-R) still works to read manually changed `.json`).

Notice, if running a simulator compiled from root you cannot load mips models. Instead start from a mips simulator, that can load mips models, e.g., the `reg_file` or `mips` examples (or the `main` in the `mips` folder). This is expected and desired behavior, a feature not a bug :)
