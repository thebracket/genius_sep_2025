# Heatmap Profiling with Flamegraph

`cargo flamegraph` makes it easy to generate interactive flamegraphs for Rust programs. It wraps Linux `perf` (or macOS `dtrace`) to sample stack traces while your program runs and then renders a collapsing stack visualization you can open in a browser.

Install prerequisites

```bash
# Install the cargo subcommand
cargo install flamegraph

# Linux: install perf (Ubuntu/Debian)
sudo apt-get update && sudo apt-get install -y linux-tools-common linux-tools-generic linux-tools-$(uname -r)

# Fedora
sudo dnf install -y perf

# Optional (Linux): allow non-root perf sampling
sudo sysctl kernel.perf_event_paranoid=1
```

Basic usage

```bash
# Profile the default binary in release mode
cargo flamegraph --release

# Profile a specific binary in a workspace
cargo flamegraph -p your_crate --bin your_binary --release

# Pass arguments to your program after `--`
cargo flamegraph --release -- --input big.json --mode hot
```

Using Docker (Mac-friendly)

Running `perf` on macOS is awkward. This repo includes a ready-to-go Docker setup in `code/flamegraph` that profiles the included allocation-heavy demo and writes the SVG to a host directory.

```bash
# 1) Build the image (from the crate directory)
docker build -t rust-flamegraph .

# 2) Create an output folder on your host
mkdir fg

# 3) Run with extra privileges so perf can sample, and mount the output dir
docker run --rm \
  --cap-add=SYS_ADMIN --security-opt seccomp=unconfined --pid=host \
  -v ./fg:/out \
  rust-flamegraph

# Optional: override workload size/iterations passed to the demo
docker run --rm \
  --cap-add=SYS_ADMIN --security-opt seccomp=unconfined --pid=host \
  -v /tmp/flamegraphs:/out \
  rust-flamegraph -- --iters 10000 --size 200000

# 4) Open the SVG on your host
open /tmp/flamegraphs/flamegraph.svg   # macOS
# xdg-open /tmp/flamegraphs/flamegraph.svg  # Linux
```

Notes

- The Docker image uses Debian’s `linux-perf` and installs `cargo flamegraph` inside the container.
- The demo program performs repeated allocations and a simple checksum in nested functions so stack frames show up clearly.
- `--pid=host` and relaxed security settings are commonly required for `perf` inside Docker. If your environment disallows them, profile directly on Linux instead.

Benchmark and example targets

```bash
# Profile a benchmark target named `prime`
cargo flamegraph -p benchmark_critereon --bench prime

# Profile an example target
cargo flamegraph --example my_example
```

Output

- Produces `flamegraph.svg` in the working directory.
- Open in a browser and use search (Ctrl/Cmd+F) to find functions; the widest boxes are your hottest paths.

Tips

- Build with debuginfo so symbol names are preserved. In `Cargo.toml` or `.cargo/config.toml`, ensure release has `debug = true` or `debug = 1`.
- Prefer `--release` to avoid profiling debug overhead.
- If symbols appear as hex addresses, install appropriate debug symbols for system libraries or enable Rust debuginfo as above.
- For noisy async stacks, use `--min-width 0.5` or `--root` to focus, and consider adding tracing spans to correlate work.

macOS note

- `cargo flamegraph` uses DTrace on macOS, which may require special permissions and can be limited on recent versions. If you see permission errors, run under `sudo` or prefer profiling on Linux where possible.
