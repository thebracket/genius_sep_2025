# WS: Spawning Your First Thread

Let's make a threaded hello world program.

Create a new directory for the project:

```bash
cargo new hello_thread
cd hello_thread
```

Now let's write the `src/main.rs` file:

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("Hello from a thread!");
    });
    println!("Hello from the main thread!");
    handle.join().unwrap();
}
```

This prints either:

```
Hello from the main thread!
Hello from a thread!
```

Or sometimes:

```
Hello from a thread!
Hello from the main thread!
```

The order of the output is not guaranteed because the two threads are running concurrently, and the operating system's scheduler determines which thread gets to run at any given time.

## Unpacking the Code

* `use std::thread;` - This imports the `thread` module from the Rust standard library, which provides functionality for creating and managing threads.
* `let handle = thread::spawn(|| { ... });` - This line creates a new thread and returns a `JoinHandle`.
* `handle.join().unwrap();` - This line waits for the spawned thread to finish executing. The `join` method returns a `Result`, which we unwrap to handle any potential errors. If the thread paniced, join will return an error - "poisoning".

> This is an exercise partly to make sure your computer/setup works - and partly because it really is that easy to spawn a thread in Rust.

Some points to note:

* If you don't join the thread, the main thread may finish executing before the spawned thread does anything. You can skip "join" if your program will continue doing other work and you want the thread to be "detached".
* `unwrap()` should probably be replaced with proper error handling in production code.
