# aleo-std-time

This crate uses `aleo-std-timer` to implement a convenient attribute to time functions.

```rust
use aleo_std::prelude::*;

#[time]
fn foo() -> u32 {
    // Insert expensive operation
    1 + 1
}
```
