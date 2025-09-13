# PyO3 Fibonacci

Let's modify our Fibonacci example to use PyO3, a Rust crate that allows us to write Python extensions in Rust.

```rust
fn fibo(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibo(n - 1) + fibo(n - 2),
    }
}
```

And we'll add an exposed function that Python can call:

```rust
#[pyfunction]
fn recur_fibo(n: u64) -> PyResult<u64> {
    Ok(fibo(n))
}
```

And we compile it with `maturin develop` as before.