# RISCV

RISCV specific components.

The ``riscv`` example will compile the assembly source file ``./asm.s``, link it using ``./memory.x`` , and initialize the instruction memory accordingly. To that end, ``riscv-gnu-toolchain`` is a hard dependency.

A sample ``asm.s`` is provided, but any instructions except opcode ``CSR`` and ``MISC-MEM`` (so CSR read/writes and FENCE instructions) are supported.

The current implementation does not initialize the data memory from the passed assembly file, this is next on the agenda.
