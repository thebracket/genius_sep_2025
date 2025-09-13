# Returning Values from Threads

Threads are like mini-programs: you probably want some input and output (not always!).

Let's modify our previous example to return a value from the thread.

```rust
use std::thread;

fn hello(n: i32) {
    println!("Hello from a thread! {}", n);
}

fn main() {
    let mut handles = vec![];
    for i in 0..5 {
        let handle = thread::spawn(move || {
            hello(i);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
```

In this example, the closure is *capturing* the variable `i` from the surrounding environment. The `move` keyword is used to indicate that the closure takes ownership of `i`, allowing it to be safely used within the new thread. In this case, it doesn't actually do anything - `i32` is a `Copy` type, which is safe to use after move. We'll see more about ownership and borrowing later.

Now let's modify the example to return a value from the thread:

```rust
use std::thread;

fn hello(n: i32) -> i32 {
    println!("Hello from a thread! {}", n);
    n * 2
}

fn main() {
    let mut handles = vec![];
    for i in 0..5 {
        let handle = thread::spawn(move || {
            hello(i) // Note we've removed the semicolon here
        });
        handles.push(handle);
    }
    for handle in handles {
        let result = handle.join().unwrap();
        println!("Thread returned: {result}");
    }
}
```