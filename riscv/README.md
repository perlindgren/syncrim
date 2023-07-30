# RISCV
RISCV specific components.

```cargo run --example riscv```
Runs the simulation with default settings.

This means compiling the assembly source file ``./asm.s``, linking it using ``./memory.x`` , and initializing the instruction and data memory accordingly. To that end, ``riscv-gnu-toolchain`` is a hard dependency.

If the riscv toolchain on your machine uses non-standard naming, the ``toolchain-prefix=$PREFIX`` can be used to change the toolchain prefix. By default, ``riscv32-unknown-elf-`` is used.

A sample ``asm.s`` is provided, but any instructions except opcodes ``CSR`` and ``MISC-MEM`` (so CSR read/writes and FENCE instructions) are supported, so experiment!

To provide your own source file or linker script, use ``asm-path=$ASM_PATH`` and ``ls-path=$LS_PATH`` respectively. The default values are ``asm.s`` and ``memory.x``.


To skip compilation and linking, the ``use-elf`` flag can be used along with ``elf-path=$ELF_PATH`` to provide the simulation with an already compiled ELF file.
