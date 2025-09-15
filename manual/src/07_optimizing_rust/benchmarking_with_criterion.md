# Benchmarking with Criterion

Criterion is a powerful benchmarking harness for Rust that provides statistical analysis, warmups, and nice reports. Here’s a minimal setup using our deliberately inefficient prime checker from the chunking exercises.

Setup (crate layout)

- Crate: `code/benchmark_critereon`
- Add Criterion as a dev-dependency and a bench target:

```toml
[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "prime"
harness = false
```

Library code (reusing the naive prime):

```rust
// code/benchmark_critereon/src/lib.rs
pub fn is_prime(n: usize) -> bool {
    if n <= 1 { false } else {
        for div in 2..n {
            if n % div == 0 { return false; }
        }
        true
    }
}

pub fn count_primes_up_to(max: usize) -> usize {
    (0..=max).filter(|&n| is_prime(n)).count()
}
```

Criterion benchmark:

```rust
// code/benchmark_critereon/benches/prime.rs
use benchmark_critereon::count_primes_up_to;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_naive_primes(c: &mut Criterion) {
    let mut group = c.benchmark_group("naive_primes");
    let max = 10_000usize; // small enough to run quickly live

    group.bench_function("count_primes_up_to_10k", |b| {
        b.iter(|| {
            let result = count_primes_up_to(black_box(max));
            black_box(result)
        })
    });

    group.finish();
}

criterion_group!(benches, bench_naive_primes);
criterion_main!(benches);
```

Run the benchmark

```bash
cargo bench -p benchmark_critereon
```

You’ll see Criterion compile a small harness and output timing with statistics. For a nicer HTML report, open `target/criterion/report/index.html` after running the benchmarks.

Tips

- Use `black_box` around inputs/outputs to avoid the compiler optimizing away your work.
- Keep workloads short so they run quickly during class; use larger sizes offline.
- Benchmark release mode (cargo does it automatically for `cargo bench`).
