# WS: Chunking Data with Rayon

A library named Rayon provides a lot of helpers for simple cases, and in many cases (such as our prime counter), you can replace your entire threading logic with a single call to `par_iter()`.

```rust
use rayon::prelude::*;

fn is_prime(n: usize) -> bool {
    if n <= 1 {
        false
    } else {
        for div in 2..n {
            if n % div == 0 {
                return false;
            }
        }
        true
    }
}

fn main() {
    const MAX_NUMBER: usize = 1000;
    let candidates: Vec<usize> = (0..MAX_NUMBER).collect();

    let primes: Vec<usize> = candidates
        .par_iter()
        .cloned()
        .filter(|&candidate| is_prime(candidate))
        .collect();

    println!("Found {} primes", primes.len());
}
```

Rayon isn't free! It will spawn one thread per CPU and a worker pool (with work stealing) to keep them busy. It's *great* for CPU-bound tasks where you don't need to worry about the details---and also don't have to worry about being nice to everything else running on the system.

Rayon is also something of a half-way-house to async. Rayon is *task* oriented, but remains thread-based. It doesn't have the lightweight nature of async tasks, but can provide a good balance between the two.