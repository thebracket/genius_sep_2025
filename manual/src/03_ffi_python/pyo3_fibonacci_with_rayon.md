# PyO3 Fibonacci with Rayon

We can go a step further and use Rayon to parallelize our Fibonacci calculation. First, add Rayon to your `Cargo.toml`:

```bash
cargo add rayon
```

And then you can add a new function to run in parallel:

```rust
#[pyfunction]
fn fibo_range(n: u64) -> PyResult<Vec<u64>> {
    use rayon::prelude::*;

    let targets: Vec<u64> = (0 .. n).collect();
    let results: Vec<u64> = targets
        .par_iter()
        .map(|n| fibo(*n))
        .collect();
    Ok(results)
}
```

Once again, add it to the pymodule:

```rust
/// A Python module implemented in Rust.
#[pymodule]
fn string_sum(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<MyClass>()?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(say_hello, m)?)?;
    m.add_function(wrap_pyfunction!(recur_fibo, m)?)?;
    m.add_function(wrap_pyfunction!(fibo_range, m)?)?;
    Ok(())
}
```

Compile it with `maturin develop` as before.