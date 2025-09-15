/// Deliberately inefficient prime checker to mirror the manual example.
/// Returns true if `n` is prime, false otherwise.
pub fn is_prime(n: usize) -> bool {
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

/// Count primes from 0..=max using the naive `is_prime`.
pub fn count_primes_up_to(max: usize) -> usize {
    (0..=max).filter(|&n| is_prime(n)).count()
}

