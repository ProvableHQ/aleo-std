# aleo-std

`aleo-std` is a standard library of tools for use in `AleoHQ` repositories.

## Usage Guide

To use this crate to your repository, add the following to your `Cargo.toml`:
```toml
[dependencies.aleo-std]
version = "0.1.0"
```

### Storage

```rust
use aleo_std::prelude::*;

fn foo() {
    println!("{:?} exists: {:?}", aleo_dir(), aleo_dir().exists());
}
```

### Time

```rust
use aleo_std::prelude::*;

#[time]
fn foo() -> u32 {
    // Insert expensive operation
    1 + 1
}
```

### Timed

```rust
use aleo_std::prelude::*;

#[timed]
fn foo(y: i32) -> i32 {
    let mut x = 1;
    let d = 1_000;
    x += d;
    x += y;
    x
}

#[timed]
fn main() {
    foo(23);
}
```

### Timer

```rust
use aleo_std::prelude::*;

fn foo() -> u32 {
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

## Roadmap

- `log`
- `std`
