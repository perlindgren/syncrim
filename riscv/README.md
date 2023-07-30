# RISCV

RISCV specific components.

The ``riscv`` example will compile the assembly source file ``./asm.s``, link it using ``./memory.x`` , and initialize the instruction and data memory accordingly. To that end, ``riscv-gnu-toolchain`` is a hard dependency.

A sample ``asm.s`` is provided, but any instructions except opcode ``CSR`` and ``MISC-MEM`` (so CSR read/writes and FENCE instructions) are supported.

```cargo run --example riscv -- --toolchain-prefix=$PREFIX```
Runs the simulation. On Linux, the --toolchain-prefix flag may be ommitted if the default ``riscv-gnu-toolchain`` is to be used.
