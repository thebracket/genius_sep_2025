# Tracing: Structured Logging

Littering your program with `println!` is effective, but it has downsides. It's also slow - println obtains a global lock (on stdout) and flushes the output on each call. This can be a significant performance hit in high-throughput applications.

The `tracing` and `tracing-subscriber` crates provide a structured logging framework that is more flexible and performant than `println!`. It allows you to log structured data, filter logs by level, and output logs in various formats (like JSON).

Emitting tracing messages is easy:

```rust
use tracing::info;
use tracing::warn;

fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    info!("divide() called with numerator: {}, denominator: {}", numerator, denominator);
    if denominator == 0.0 {
        warn!("denominator is zero, returning None");
        None
    } else {
        let result = numerator / denominator;
        info!("denominator is non-zero, returning Some({})", result);
        Some(result)
    }
}
```

You have to use the `tracing-subscriber` crate to set up a subscriber that processes the tracing events. Here's a simple example that logs to stdout:

```rust
use tracing::info;
use tracing::warn;

fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    info!("divide() called with numerator: {}, denominator: {}", numerator, denominator);
    if denominator == 0.0 {
        warn!("denominator is zero, returning None");
        None
    } else {
        let result = numerator / denominator;
        info!("denominator is non-zero, returning Some({})", result);
        Some(result)
    }
}

fn main() {
    tracing_subscriber::fmt::init();
    divide(10.0, 2.0);
    divide(10.0, 0.0);
}
```