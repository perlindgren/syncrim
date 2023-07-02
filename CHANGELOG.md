# Changelog

Tracking changes per date:

## 230702

- `path` no longer stored in `.json` model.
- `path` passed as `&PathBuf` to `gui` and `component_store`.
- `path` stored as `PathBuf` in the `gui`.

Finally, SyncRim can now open models through `Open` (CTRL-O) in `gui` (`ReOpen` (Ctrl-R) still works to read manually changed `.json`).
