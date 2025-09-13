# Async and Sync - Together

So what if the core of your program is synchronous number crunching, but you also want to handle incoming requests (and maybe outgoing requests) asynchronously? This is a common scenario in many applications.

Channels are a great way to bridge the async and sync worlds. You could use `spawn_blocking` and have a generally program - but that quickly turns into spaghetti. Fortunately, tokio's `mpsc` channel has `blocking_send` and `blocking_recv` methods that can be used in sync code.

```rust
use tokio::sync::mpsc;
use std::thread;

fn async_land(mut rx: mpsc::Receiver<i32>) {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(async {
        // You can spawn other tasks here if you want
        while let Some(value) = rx.recv().await {
            println!("Received: {}", value);
        }
    });
}

fn main() {
    let (tx, rx) = mpsc::channel(1024);

    // Spawn the async runtime in a separate thread
    let handle = thread::spawn(move || {
        async_land(rx);
    });

    // In the main thread, we can do blocking work
    for i in 1..=10 {
        // Send a message to the async runtime
        tx.blocking_send(i).unwrap();
        // Simulate some blocking work
        thread::sleep(std::time::Duration::from_millis(100));
    }

    // Close the channel
    drop(tx);

    // Wait for the async runtime to finish
    handle.join().unwrap();
}
```
