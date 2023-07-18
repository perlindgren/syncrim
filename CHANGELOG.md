# Changelog

Tracking changes per date:

## 230718

- Panes, right click component to display component interior on left panel. Close left panel view by the Close button.

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
