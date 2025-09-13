# Wrap Up: System Threads

Rust has first-class support for system threads. You can spawn them, return values from them, and share data between them. Rust's ownership model and type system help you avoid common pitfalls like data races. Deadlocks still require a bit of care, and you have to think about the *logic* of your program---but it's very difficult to accidentally corrupt some data!