# PyO3 Classes

You can also expose Rust structs as Python classes. Let's create a simple class that holds a string and has a method to return its length.

```rust
#[pyclass]
struct MyClass {
    #[pyo3(get)]
    data: i32,
}

#[pymethods]
impl MyClass {
    #[new]
    pub fn new(data: i32) -> MyClass {
        MyClass { data }
    }
}
```