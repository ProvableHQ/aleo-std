# aleo-std-storage

This crate uses `aleo-std-storage` to implement convenience methods for accessing resources in Aleo storage.

```rust
use aleo_std::prelude::*;

fn foo() {
    // Prints the Aleo directory.
    println!("{:?} exists: {:?}", aleo_dir(), aleo_dir().exists());
    // Prints the Aleo ledger directory in production mode.
    println!("{:?} exists: {:?}", aleo_ledger_dir(2, None), aleo_ledger_dir(2, None).exists());
}
```
