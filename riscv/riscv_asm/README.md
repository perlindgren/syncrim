# riscv_asm

Experiment to check if we can do assembly programming in an easy way using the Rust toolchain only.

## Installation

- Rust installed.

- RISCV backend.
  
- Rust LLVM tools distribution (you can also use any llvm or gnu counterpart)

- [Cargo binutils](https://github.com/rust-embedded/cargo-binutils) for easy integration.

```shell
rustup target add riscv32i-unknown-none-elf
rustup component add llvm-tools 
cargo install cargo-binutils
```

## Example

To build a binary.

```shell
cargo build 
```

To disassemble the generated binary.

```shell
cargo objdump -- --disassemble --print-imm-hex
```

To get size information and layout for sections and symbols.

```shell
cargo nm
```
