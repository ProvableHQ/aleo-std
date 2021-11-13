# aleo-std-storage

This crate uses `aleo-std-storage` to implement convenience methods for accessing resources in Aleo storage.

```rust
use aleo_std::prelude::*;

fn foo() {
    println!("{:?} exists: {:?}", aleo_dir(), aleo_dir().exists());
}
```
