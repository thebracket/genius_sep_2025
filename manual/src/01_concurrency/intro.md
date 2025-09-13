# Fearless Concurrency

"Fearless Concurrency" is a top advertising term for Rust --- but what does it really mean?

Rust is designed to make *data races* impossible at compile time. *Logical* races (where you just didn't think about the control flow) are still possible, but Rust's ownership system makes it practically impossible to share mutable state between threads without using synchronization primitives.

A second part of "fearless concurrency" is making it easy to write concurrent code. Concurrency is *always* hard, but Rust's standard library and ecosystem provide tools to make it easier.