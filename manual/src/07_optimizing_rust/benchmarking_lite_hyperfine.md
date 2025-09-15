# Benchmarking Lite: Hyperfine

`hyperfine` is a handy CLI tool for quick, statistically sound benchmarking of command-line programs. It runs commands multiple times, includes warmups, and reports mean, standard deviation, and confidence intervals.

Install `hyperfine`:

```bash
# macOS (Homebrew)
brew install hyperfine

# Ubuntu/Debian
sudo apt-get update && sudo apt-get install -y hyperfine

# Or via Cargo
cargo install hyperfine
```

Example: benchmark a simple program

Assume you have a program `my_app` you want to benchmark. You can compare debug vs. release or different modes/flags.

```bash
# Compare debug vs. release builds
hyperfine --warmup 3 'target/debug/my_app' 'target/release/my_app'

# Compare different runtime flags
hyperfine --warmup 3 'my_app --mode baseline' 'my_app --mode optimized'
```

Useful options:

```bash
# Run a setup step before each benchmarked command (e.g., clear cache/files)
hyperfine --prepare 'rm -f out.txt' 'my_app --input data.json'

# Increase runs (defaults are usually fine, but you can raise them)
hyperfine -w 3 -m 20 'my_app --mode optimized'

# Export results for reporting
hyperfine 'my_app' --export-json results.json
hyperfine 'my_app' --export-markdown results.md
```

Tip: Always benchmark release builds for realistic performance. Use `cargo build --release` first and point `hyperfine` to `target/release/your_binary`.
