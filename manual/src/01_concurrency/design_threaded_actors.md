# Design 2: Threaded Actors

So let's revisit design. The last design we looked at isolated your state into a module, used a private Mutex to protect it, and provided nice accessor functions to interact with it. Not a bad design. A very common design in the Rust world is to use channels to make an "actor" (just like Erlang, Elixir, etc.). The thread has exclusive ownership of the state it protects, and you send messages to interact with it.

```rust
// Pretend this is a separate module or even crate
mod data {
    use tokio::sync::oneshot;
    use std::sync::mpsc;
    use std::thread;
    use std::sync::OnceLock;

    enum Command {
        GetValue(oneshot::Sender<i32>),
        SetValue(i32),
    }

    static TX: OnceLock<mpsc::Sender<Command>> = OnceLock::new();

    pub fn start() {
        let (tx, rx) = mpsc::channel();
        TX.set(tx).unwrap();
        thread::spawn(move || {
            let mut value = 0;
            for command in rx {
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

    pub fn get_value() -> i32 {
        let (reply_tx, reply_rx) = oneshot::channel();
        TX.get().unwrap().send(Command::GetValue(reply_tx)).unwrap();
        reply_rx.blocking_recv().unwrap()
    }

    pub fn set_value(new_value: i32) {
        TX.get().unwrap().send(Command::SetValue(new_value)).unwrap();
    }
}

fn main() {
    data::start();
    data::set_value(42);
    let value = data::get_value();
    println!("Value: {}", value);
}
```

Not the most spectacular demo, but it has a lot of advantages:

* The state is completely isolated in its own thread. No other thread can access it directly.
* The state is not protected by a Mutex, so there's no risk of deadlocks.
* The accessor functions are simple and easy to use.
* You can easily add more commands to the `Command` enum to extend the functionality.
* You can easily change the implementation of the actor without affecting the rest of the code.

You can even use an `mpmc` channel (Multiple Producer, Multiple Consumer) --- such as `crossbeam::channel` --- to have multiple instances of the actor running in parallel (for genuinely shared state you'll need to use a `Mutex` or other synchronization primitive).

And most importantly: by strictly controlling the access API, your client doesn't need to know how it works. That's *really* handy if you realize that the service needs to be promoted to a network service later!