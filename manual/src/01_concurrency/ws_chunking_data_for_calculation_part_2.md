# WS: Part 2 Chunking Data for Calculation

Now let's use what we've learned, and make this prime number finder use multiple threads to speed things up.

```rust
use std::sync::{Mutex};
use std::thread::available_parallelism;
use std::thread;

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
    let mut primes: Mutex<Vec<usize>> = Mutex::new(Vec::new());

    let num_cpus = available_parallelism().map_or(4, |n| n.get());
    let chunks = candidates.chunks(candidates.len() / num_cpus + 1);

    thread::scope(|s| {
        for chunk in chunks {
            let chunk = chunk.to_vec();
            let primes = &primes;
            s.spawn(move || {
                for candidate in chunk {
                    if is_prime(candidate) {
                        primes.lock().unwrap().push(candidate);
                    }
                }
            });
        }
    });

    println!("Found {} primes", primes.lock().unwrap().len());
}
```