# Async Mutexes

One way in which people often get into trouble in Async land is when they need a Mutex. The standard library mutex can block the entire thread, which can be bad in an async context. If you need a mutex in async code, you should use an async-aware mutex.

Tokio provides an async mutex in the `tokio::sync` module. It works similarly to the standard library mutex, but its `lock` method is async and returns a future that you can `await`.

```rust
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let mutex = Mutex::new(0);

    let lock = mutex.lock().await;
    *lock += 1;
}
```

If you wrap it an an `Arc`, it can be cloned as needed between tasks.

```rust
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task;

#[tokio::main]
async fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(task::spawn(async move {
            let mut num = counter.lock().await;
            *num += 1;
        }));
    }
    for handle in handles {
        handle.await.unwrap();
    }
    println!("Result: {}", *counter.lock().await);
}
```

## Footguns!

Holding a lock across an `await` boundary is a *really bad idea*. Tokio can avoid a deadlock in most cases nowadays, but it's still a bad idea. What if the task holding the lock gets suspended for a long time? What if it is canceled? What if it panics? All of these can lead to performance issues or even deadlocks.

So just like a normal Mutex: hold it for as short a time as possible, and avoid holding it across an `await` boundary (just like you would avoid holding a normal Mutex across a blocking operation).