use std::env;
use std::time::Instant;

fn hot_allocation(size: usize) -> u64 {
    // Allocate a buffer, fill it, and compute a checksum to keep work alive.
    let mut v = Vec::with_capacity(size);
    v.resize(size, 42u8);
    v.iter().map(|&b| b as u64).sum()
}

fn stage_b(iters: usize, size: usize) -> u64 {
    let mut acc = 0u64;
    for _ in 0..iters {
        acc = acc.wrapping_add(hot_allocation(size));
    }
    acc
}

fn stage_a(iters: usize, size: usize) -> u64 {
    stage_b(iters, size)
}

fn parse_arg(flag: &str, default: usize) -> usize {
    // Parse from CLI args like --iters 5000 or --size 100000, falling back to env and default
    let args: Vec<String> = env::args().collect();
    if let Some(pos) = args.iter().position(|a| a == flag) {
        if let Some(val) = args.get(pos + 1) {
            if let Ok(n) = val.parse::<usize>() {
                return n;
            }
        }
    }
    let env_key = match flag {
        "--iters" => "ITERS",
        "--size" => "SIZE",
        _ => "",
    };
    if !env_key.is_empty() {
        if let Ok(val) = env::var(env_key) {
            if let Ok(n) = val.parse::<usize>() {
                return n;
            }
        }
    }
    default
}

fn main() {
    // Defaults chosen to produce visible allocator/CPU activity quickly.
    let iters = parse_arg("--iters", 5_000);
    let size = parse_arg("--size", 100_000); // ~100 KB per iteration

    eprintln!("Starting workload: iters={}, size={} bytes", iters, size);
    let start = Instant::now();
    let checksum = stage_a(iters, size);
    let elapsed = start.elapsed();
    println!(
        "Finished. checksum={}, elapsed_ms={}",
        checksum,
        elapsed.as_millis()
    );
}
