# Fearless Concurrency with Rust

In this case, the correct way to write this is to use an *atomic*. Atomics are special types (CPU instructions) that allow you to read and write values in a way that is guaranteed to be safe across threads. There's a small performance penalty to using atomics, but they are the right tool for this job.

Here's how you would use an atomic to safely increment a counter from multiple threads:

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

static COUNTER: AtomicUsize = AtomicUsize::new(0);

fn main() {
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                COUNTER.fetch_add(1, Ordering::SeqCst);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter value: {}", COUNTER.load(Ordering::SeqCst));
}
```

A Mutex isn't really the right tool for this job, but it works too.

```rust
use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::LazyLock;

static COUNTER: LazyLock<Arc<Mutex<usize>>> = LazyLock::new(|| Arc::new(Mutex::new(0)));

fn main() {
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&COUNTER);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                let mut num = counter.lock().unwrap();
                *num += 1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter value: {}", COUNTER.lock().unwrap());
}
```

This is getting ugly! We're using `LazyLock` to provide lazy initialization of the static `Mutex`, which is wrapped in an `Arc`.

> An `Arc` is an *atomic reference counted* pointer. It's basically garbage collection for Rust.