# Using the Debug Trait

You can decorate most types with the `#[derive(Debug)]` attribute to automatically implement the `Debug` trait. This allows you to use the `{:?}` format specifier in `println!` and other formatting macros to print the value in a human-readable format.

Here's an example:

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 10, y: 20 };
    println!("Point: {:?}", p);
    println!("Point with pretty print: {:#?}", p);
}
```

If you want to modify Debug to - for example - not print certain fields, you can implement the `Debug` trait manually:

```rust
use std::fmt;
struct Point {
    x: i32,
    y: i32,
    secret: String,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Point")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

fn main() {
    let p = Point { x: 10, y: 20, secret: "classified".to_string() };
    println!("Point: {:?}", p);
}
```