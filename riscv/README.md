# RISCV

RISCV specific components.

For now this requires nightly to compile the included assembly.

## Initializing the model with assembly

```cargo run --example riscv```
Runs the simulation with default settings.

This means compiling the assembly source file ``./asm.s``, linking it using ``./memory.x`` , and initializing the instruction and data memory accordingly. This is all done using cargo, meaning no dependencies except the ``riscv32i-unknown-none-elf`` target. It may be installed via ``rustup target add riscv32i-unknown-none-elf``

A sample ``asm.s`` is provided, but any instructions except opcode ``MISC-MEM`` (so FENCE instructions) are supported, so experiment!

To provide your own source file or linker script, use ``asm-path=$ASM_PATH`` and ``ls-path=$LS_PATH`` respectively. The default values are ``asm.s`` and ``memory.x``.

To skip compilation and linking, the ``use-elf`` flag can be used along with ``elf-path=$ELF_PATH`` to provide the simulation with an already compiled ELF file.

## Initializing the model with Rust

```cargo run --example riscv -- --rust```
Runs the simulation with the Rust flag.

This means compiling the crate ``riscv-basic`` in ``./``, linking it using ``./memory.x`` and initializing the instruction and data memory.

Similarly to the assembly example, this is all done automatically using cargo, meaning no additional dependencies except the ``riscv32i-unknown-none-elf`` target.

A sample application is provided, in this case a simple [``RTIC``](https://github.com/rtic-rs/rtic/) application which spawns a task ``foo``, that spawns another task ``baz`` . ``baz`` does nothing and returns, and upon that ``foo`` loops infintely.

The Rust support for the ``CLIC`` and the ``RTIC`` support for CLIC-equipped RISC-V CPUs is in an early work in progress stage, driven by the development of the SyncRim RISC-V model.
