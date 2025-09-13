# What's Really Happening?

There's an awful lot of magic in that "hello world". Let's break it down.

The Tokio main macro is actually writing:

```rust
async fn say_hello() {
    println!("Hello, world!");
}

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        say_hello().await;
    });
}
```

This is important, because if you are mixing threads and async, you often want to create your own thread and put the runtime inside it. You can even make the runtime single-threaded if you want to:

```rust
let rt = tokio::runtime::Builder::new_current_thread()
    .enable_all()
    .build()
    .unwrap();
```

The `block_on` function creates a new task, runs it, and blocks the current thread until it completes. Inside that task, we call `say_hello().await`, which prints "Hello, world!".

> You can't call `async` functions from normal/sync code. This "function coloring" problem is quite widely discused!