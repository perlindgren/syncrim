# Github workflow breakdown

The workflow starts with checking out the repo, installing the required dependencies (`librust-atk-dev`, `librust-gdk-dev`, the Rust toolchain, and the Mold linker). It then restores the cache stored by `Swatinem/rust-cache@v2`. This includes the `~/.cargo` and `./target` directories, which contain installed binaries, the cargo registry, the cache, git dependencies and build artifacts of dependencies.

Next, a build check is ran via `cargo build --verbose`, followed by clippy checks on the `gui-vizia` and `gui-egui` versions of the crate, and finally, the tests are ran via `cargo test --workspace --no-default-features --features components --verbose`.

As a final step, the `Swatinem/rust-cache@v2` workflow updates the aforementioned cache for re-use by the next job.

In parallel, a rustfmt check is ran via ``cargo fmt --all --check``. This ensures that all of the Rust code is formatted with ``cargo fmt``.