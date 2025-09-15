## Build and profile this crate with cargo flamegraph inside Docker.

FROM rust:1.89-bookworm as base
WORKDIR /work

# Install perf tools and cargo-flamegraph
RUN apt-get update \
    && apt-get install -y --no-install-recommends linux-perf ca-certificates \
    && update-ca-certificates \
    && cargo install flamegraph \
    && rm -rf /var/lib/apt/lists/*

# Cache deps sensibly by copying manifest first
COPY Cargo.toml ./
RUN mkdir -p src && echo "fn main(){}" > src/main.rs && cargo build || true

# Now copy the real sources
COPY src ./src

# Build release binary (optional; flamegraph will rebuild if needed)
RUN cargo build

# By default, run cargo flamegraph and write to /out/flamegraph.svg
# You can override iters/size by passing flags after -- when running the container
CMD cargo flamegraph --bin flamegraph --output /out/flamegraph.svg -- --iters 5000 --size 100000

