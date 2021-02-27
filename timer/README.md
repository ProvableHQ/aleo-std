# aleo-std-timer

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
