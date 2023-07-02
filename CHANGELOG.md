# Changelog

Tracking changes per date:

## 230702

- `path` no longer stored in `.json` model.
- `path` passed as `&PathBuf` to `gui` and `component_store`.
- `path` stored as `PathBuf` in the `gui`.

Finally, SyncRim can now open models through `Open` (CTRL-O) in `gui` (`ReOpen` (Ctrl-R) still works to read manually changed `.json`).

Notice, if running a simulator compiled from root you cannot load mips models. Instead start from a mips simulator, that can load mips models, e.g., the `reg_file` or `mips` examples (or the `main` in the `mips` folder). This is expected and desired behavior, a feature not a bug :)
