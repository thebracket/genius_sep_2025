# Tracing: JSON Output

If you use a log capturing system, it often helps to use JSON output for logging.

Modify the features of `tracing-subscriber` in `Cargo.toml` to include `json`:

```toml
tracing-subscriber = { version = "0.3.20", features = ["fmt", "json"] }
```

Then modify the `init_tracing` function in `src/main.rs` to use the JSON formatter:

```rust
tracing_subscriber::fmt()
    .with_max_level(Level::INFO)
    .with_target(true)
    .with_file(true)
    .with_line_number(true)
    .with_thread_ids(true)
    .with_thread_names(true)
    .with_span_events(FmtSpan::CLOSE)
    .with_timer(SystemTime)
    .json()
    .init();
```