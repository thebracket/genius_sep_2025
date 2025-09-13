# Release Builds

You'd be *amazed* at how many questions to `r/rust` (Reddit) and Stack Overflow are answered with "Did you try a release build?" The answer is almost always "No."

By default, `cargo build` creates a debug build. Debug builds are not optimized, and are much slower than release builds. To create a release build, use the `--release` flag:

```bash
cargo build --release
cargo run --release # Or run it directly
```

Release builds are optimized for speed, and are typically much faster than debug builds. The trade-off is that release builds take longer to compile.

There's a few other differences between debug and release builds:
* Debug builds include debug symbols, which are useful for debugging. Release builds do not include debug symbols by default, but you can enable them in your `Cargo.toml` if needed.
* Debug builds have overflow checks enabled, which can help catch bugs. Release builds have overflow checks disabled by default, but you can enable them in your `Cargo.toml` if needed.
* Debug builds have less aggressive inlining, which can make debugging easier. Release builds have more aggressive inlining, which can improve performance.