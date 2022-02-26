# aleo-std-time

[![Crates.io](https://img.shields.io/crates/v/aleo-std-time.svg?color=neon)](https://crates.io/crates/aleo-std-time)
[![Authors](https://img.shields.io/badge/authors-Aleo-orange.svg)](https://aleo.org)
[![License](https://img.shields.io/badge/License-GPLv3-blue.svg)](./LICENSE.md)

This crate uses `aleo-std-time` to implement a convenient attribute to time functions.

```rust
use aleo_std::prelude::*;

#[time]
fn foo() -> u32 {
    // Insert expensive operation
    1 + 1
}
```
