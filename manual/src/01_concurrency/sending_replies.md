# Sending Replies

When using channels, you can also create a channel for sending replies back to the original sender. This is useful when you want to send a request to a worker thread and get a response back. The `tokio::oneshot` crate is the de-facto standard for this.

```rust
use std::sync::mpsc;
use std::thread;
use tokio::sync::oneshot;

enum Command {
    SayHello(String, oneshot::Sender<String>),
}

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        for command in rx {
            match command {
                Command::SayHello(name, reply_tx) => {
                    let greeting = format!("Hello, {}!", name);
                    reply_tx.send(greeting).unwrap();
                }
            }
        }
    });

    let names = vec!["Alice", "Bob", "Charlie"];
    for name in names {
        let (reply_tx, reply_rx) = oneshot::channel();
        tx.send(Command::SayHello(name.to_string(), reply_tx)).unwrap();
        let greeting = reply_rx.blocking_recv().unwrap();
        println!("{}", greeting);
    }
}
```

So now you have a way to call into a thread and get a response back. This is great for building request/response systems. You can mix and match your enum commands to have some commands that don't expect a reply and some that do.