# Type State Patterns

```rust
use std::marker::PhantomData;

// Marker types for states
struct Closed;
struct Open;

// Generic over the state type
struct Door<S>(PhantomData<S>);

impl Door<Closed> {
    fn open(self) -> Door<Open> { Door(PhantomData) }
}

impl Door<Open> {
    fn close(self) -> Door<Closed> { Door(PhantomData) }
    fn enter(&self) {} // only valid when open
}

fn main() {
    let d = Door::<Closed>(PhantomData);
    let d = d.open();
    d.enter();
    let _d = d.close();
}
```

The `PhantomData` type is a zero-sized type that indicates that our `Door` struct is generic over the state type `S`, even though we don't actually use `S` in the struct. This allows us to create different types for each state of the door (`Door<Closed>` and `Door<Open>`), and the Rust compiler will enforce the state transitions at compile time.

> The key here is that you are aiming to make impossible states unrepresentable. If you try to call `enter` on a `Door<Closed>`, or `close` on a `Door<Closed>`, the code won't compile. This is a powerful way to use Rust's type system to enforce correct usage patterns in your code.