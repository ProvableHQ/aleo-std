# aleo-std-timed

This crate implements a profiler to conveniently time function executions.

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
