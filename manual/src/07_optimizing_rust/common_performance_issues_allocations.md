# Allocations

Allocations are one of the most common performance pitfalls in Rust. This is particularly true when working with collections.

This program allocates regularly:

```rust
fn main() {
    let mut v = Vec::new();
    for i in 0..1_000_000 {
        v.push(i);
    }
}
```

Allocation occurs everytime the vector needs to grow. By default, a `Vec` will double its capacity each time it runs out of space. This means that the number of allocations is logarithmic in relation to the number of elements.

Here's the same program, but with a pre-allocated vector:

```rust
fn main() {
    let mut v = Vec::with_capacity(1_000_000);
    for i in 0..1_000_000 {
        v.push(i);
    }
}
```

> The `collect()` method also allocates, but it does so only once, because it can determine the size of the collection ahead of time. Other containers such as `HashMap` and `HashSet` also have `with_capacity` methods.

## Related: Pointer Chasing

If you allocate items to the heap, then accessing them becomes a two-step process: read the pointer, and then follow it to the heap. This is called pointer chasing, and it can be a performance issue because it can lead to cache misses.

For example, consider this program:

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    for i in 0..1_000_000 {
        map.insert(i, Box::new(i));
    }

    let mut sum = 0;
    for i in 0..1_000_000 {
        sum += *map.get(&i).unwrap();
    }
    println!("Sum: {}", sum);
}
```

Here, we allocate a `Box` for each value in the `HashMap`. This means that when we access the values, we have to follow a pointer to the heap for each value. This can lead to a lot of cache misses, which can slow down the program.

## Read Forwards

When accessing data, try to access it in a linear fashion. This helps the CPU prefetch data into the cache, which can significantly speed up access times.
