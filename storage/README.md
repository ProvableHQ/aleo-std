# aleo-std-storage

[![Crates.io](https://img.shields.io/crates/v/aleo-std-storage.svg?color=neon)](https://crates.io/crates/aleo-std-storage)
[![Authors](https://img.shields.io/badge/authors-Aleo-orange.svg)](https://aleo.org)
[![License](https://img.shields.io/badge/License-GPLv3-blue.svg)](./LICENSE.md)

This crate uses `aleo-std-storage` to implement convenience methods for accessing resources in Aleo storage.

```rust
use aleo_std::prelude::*;

fn foo() {
    // Prints the Aleo directory.
    println!("{:?} exists: {:?}", aleo_dir(), aleo_dir().exists());
    // Prints the Aleo ledger directory in production mode.
    println!("{:?} exists: {:?}", aleo_ledger_dir(2, None), aleo_ledger_dir(2, None).exists());
    // Prints the Aleo operator directory in production mode.
    println!("{:?} exists: {:?}", aleo_operator_dir(2, None), aleo_operator_dir(2, None).exists());
    // Prints the Aleo prover directory in production mode.
    println!("{:?} exists: {:?}", aleo_prover_dir(2, None), aleo_prover_dir(2, None).exists());
}
```
