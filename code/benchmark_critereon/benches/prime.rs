use benchmark_critereon::count_primes_up_to;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_naive_primes(c: &mut Criterion) {
    let mut group = c.benchmark_group("naive_primes");

    // Choose a modest upper bound so the benchmark runs fast in class.
    let max = 10_000usize;

    group.bench_function("count_primes_up_to_10k", |b| {
        b.iter(|| {
            // black_box to prevent the compiler from optimizing away the work
            let result = count_primes_up_to(black_box(max));
            black_box(result)
        })
    });

    group.finish();
}

criterion_group!(benches, bench_naive_primes);
criterion_main!(benches);

