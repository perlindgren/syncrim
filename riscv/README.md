# RISCV

RISCV specific components.

```cargo run --example riscv```
Runs the simulation with default settings.

This means compiling the assembly source file ``./asm.s``, linking it using ``./memory.x`` , and initializing the instruction and data memory accordingly. This is all done using cargo, meaning no dependencies except the ``riscv32i-unknown-none-elf`` target. It may be installed via ``rustup target add riscv32i-unknown-none-elf``

A sample ``asm.s`` is provided, but any instructions except opcodes ``CSR`` and ``MISC-MEM`` (so CSR read/writes and FENCE instructions) are supported, so experiment!

To provide your own source file or linker script, use ``asm-path=$ASM_PATH`` and ``ls-path=$LS_PATH`` respectively. The default values are ``asm.s`` and ``memory.x``.

To skip compilation and linking, the ``use-elf`` flag can be used along with ``elf-path=$ELF_PATH`` to provide the simulation with an already compiled ELF file.
