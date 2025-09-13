# Design 3: Async Actors

The actor model we talked about with threads has one problem: it uses a lot of threads. Async tasks are really lightweight, and you're quite unlikely to ever run out of them.

This makes channels and actors a *great* fit for async code. Tokio includes several async channel implementations, including both `mpsc` and `oneshot`. So let's make our example from earlier into an async actor.

```rust
// Pretend this is a separate module or even crate
mod data {
    use tokio::sync::oneshot;
    use tokio::sync::mpsc;
    use std::sync::OnceLock;

    enum Command {
        GetValue(oneshot::Sender<i32>),
        SetValue(i32),
    }

    // We're using Tokio's OnceLock here, which is async-aware
    static TX: OnceLock<mpsc::Sender<Command>> = OnceLock::new();

    pub async fn start() {
        // Tokio always bounds channels (good)!
        let (tx, mut rx) = mpsc::channel(1024);
        TX.set(tx).unwrap();
        tokio::spawn(async move {
            let mut value = 0;
            // Receive is similar - but async
            while let Some(command) = rx.recv().await {
                match command {
                    Command::GetValue(reply_tx) => {
                        let _ = reply_tx.send(value);
                    }
                    Command::SetValue(new_value) => {
                        value = new_value;
                    }
                }
            }
        });
    }

    pub async fn get_value() -> i32 {
        let (reply_tx, reply_rx) = oneshot::channel();
        TX.get().unwrap().send(Command::GetValue(reply_tx)).await;
        reply_rx.await.unwrap()
    }

    pub async fn set_value(new_value: i32) {
        TX.get().unwrap().send(Command::SetValue(new_value)).await;
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    data::start().await;
    data::set_value(42).await;
    let value = data::get_value().await;
    println!("Value: {}", value);
}
```

And now we have the same thing - but only using one thread. You keep the isolation and simplicity of the actor model, but without the overhead of threads.