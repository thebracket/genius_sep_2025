# Channels and Thread Communication

> Rob Pike - of Go language fame - once said, "Do not communicate by sharing memory; instead, share memory by communicating."

Channels are actually a lot older than Go (dating to at least the 1970s). They provide a mechanism for sending data between threads. MPSC (Multi Producer, Single Consumer) are built into the Rust standard library.

Let's send an integer from one thread to another.

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        for i in 1..10 {
            tx.send(i).unwrap();
            println!("Sent {}", i);
        }
    });

    for received in rx {
        println!("Received {}", received);
    }
}
```

You don't have to just use integers! Sending enumerations as a "command pattern" is a common use case.

```rust
use std::sync::mpsc;
use std::thread;

enum Command {
    SayHello(String),
}

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let names = vec!["Alice", "Bob", "Charlie"];
        for name in names {
            tx.send(Command::SayHello(name.to_string())).unwrap();
            println!("Sent greeting for {}", name);
        }
    });

    for command in rx {
        match command {
            Command::SayHello(name) => println!("Hello, {}!", name),
        }
    }
}
```

You can even share function pointers (which is how a lot of thread pool libraries work).

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let tasks: Vec<Box<dyn FnOnce() + Send>> = vec![
            Box::new(|| println!("Task 1 executed")),
            Box::new(|| println!("Task 2 executed")),
            Box::new(|| println!("Task 3 executed")),
        ];

        for task in tasks {
            tx.send(task).unwrap();
            println!("Sent a task");
        }
    });

    for task in rx {
        task();
    }
}
```
