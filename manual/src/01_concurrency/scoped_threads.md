# Scoped Threads

It was pretty nasty putting everything in a static! If you're working with local data, you can use scoped threads to avoid all the `Arc` and `LazyLock` ugliness.

```rust
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0);

    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                for _ in 0..1000 {
                    let mut num = counter.lock().unwrap();
                    *num += 1;
                }
            });
        }
    });

    println!("Final counter value: {}", counter.lock().unwrap());
}
```