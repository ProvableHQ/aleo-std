# aleo-std-cpu

This crate uses `aleo-std-cpu` to implement convenience methods for retrieving CPU information.

Note: This crate only supports Intel and AMD chipsets. For performance reasons in snarkVM, M1 chips default to Intel.

```rust
fn foo() {
    // Prints the CPU name.
    println!("{:?}", aleo_std::get_cpu());
}
```
