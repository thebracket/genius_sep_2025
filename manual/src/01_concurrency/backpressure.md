# Backpressure

The default channel we made was unbounded. This means that the sender can send as many messages as it wants without waiting for the receiver to receive them. This can lead to high memory usage if the sender is producing messages faster than the receiver can consume them.

Let's create a bounded channel with a capacity of 2. This means that the sender can only send 2 messages before it has to wait for the receiver to receive them.

```rust
use std::sync::mpsc::sync_channel;
use std::thread;

fn main() {
    let (tx, rx) = sync_channel(2); // Bounded channel with capacity of 2
    thread::spawn(move || {
        for i in 1..10 {
            tx.send(i).unwrap();
            println!("Sent {}", i);
        }
    });

    for received in rx {
        println!("Received {}", received);
        thread::sleep(std::time::Duration::from_secs(1)); // Simulate slow processing
    }
}
```

The sender will block (thread pause) whenever the channel is full. In this case, the sender will block after sending 2 messages until the receiver receives at least one message.

> This is *great* if you're building a modular monolith with modules connected by channels (and even better if you've written proper accessor functions for the channels). Each module can run at its own pace, and if one module is slow, it won't overwhelm the others.