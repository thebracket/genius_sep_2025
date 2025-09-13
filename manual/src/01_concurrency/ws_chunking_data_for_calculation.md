# WS: Chunking Data for Calculation

A *really* common task with threads is to break up a large data set into smaller chunks, and have multiple threads process those chunks in parallel.

Let's start with a really inefficient prime number finder:

```rust
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
    let mut primes: Vec<usize> = Vec::new();

    for candidate in candidates {
        if is_prime(candidate) {
            primes.push(candidate);
        }
    }

    println!("Found {} primes", primes.len());
}
```
