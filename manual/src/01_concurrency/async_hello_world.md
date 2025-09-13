# Async Hello World

Let's make an async "hello world".

```bash
cargo new async-hello-world
cd async-hello-world
cargo add tokio --features full
```

Now edit `src/main.rs`:

```rust
async fn say_hello() {
    println!("Hello, world!");
}

#[tokio::main]
async fn main() {
    say_hello().await;
}
```

If you omit the `.await` you get a compiler warning - and nothing happens. Any function decorated with `async` is actually transformed into a state machine that implements the `Future` trait. Calling an async function does not execute it - it just creates the state machine. You have to `.await` it to actually run it.