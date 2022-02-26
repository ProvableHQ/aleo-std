# aleo-std-timer

[![Crates.io](https://img.shields.io/crates/v/aleo-std-timer.svg?color=neon)](https://crates.io/crates/aleo-std-timer)
[![Authors](https://img.shields.io/badge/authors-Aleo-orange.svg)](https://aleo.org)
[![License](https://img.shields.io/badge/License-GPLv3-blue.svg)](./LICENSE.md)

This crate implements a straightforward timer to conveniently time code blocks.

```rust
use aleo_std::prelude::*;

fn foo() -> u32{
    // Start the timer.
    let timer = timer!("Arithmetic");

    // Insert expensive operation
    let x = 1 + 1;

    // Print the elapsed time up to this point.
    lap!(timer);

    // Insert expensive operation
    let y = 1 + 1;

    // Print the total time elapsed.
    finish!(timer);

    x + y
}
```
