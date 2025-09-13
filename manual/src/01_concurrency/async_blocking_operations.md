# Async Blocking Operations

Async Rust is pretty amazing, but it has a footgun. If you "block" inside an async function, you block the entire thread. 

```rust
#[tokio::main]
async fn main() {
    // This will block the entire thread
    std::thread::sleep(std::time::Duration::from_secs(1));
}
```

While that thread is sleeping, no tasks in its work queue can execute - and work stealing may not even work, because the thread is blocked. This can lead to performance issues and deadlocks. (In this case, Tokio provides an async sleep function that we've used before).

You can also cause Tokio to grind to a halt by performing CPU-intensive work in an async function.

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

#[tokio::main]
async fn main() {
    let mut handles = vec![];
    for i in 1..=10 {
        handles.push(tokio::spawn(async move {
            let prime = is_prime(i);
            println!("{} is prime: {}", i, prime);
        }));
    }
    for handle in handles {
        handle.await.unwrap();
    }
}
```

It works fine, but if you had other work to do---like handling incoming requests in a server---it would be blocked until all the prime calculations were done.

You have some options!

## Option 1: Async and Yield

```rust
async fn is_prime(n: usize) -> bool {
    if n <= 1 {
        false
    } else {
        for div in 2..n {
            if n % div == 0 {
                return false;
            }
            tokio::task::yield_now().await; // Yield to the runtime
        }
        true
    }
}

#[tokio::main]
async fn main() {
    let mut handles = vec![];
    for i in 1..=10 {
        handles.push(tokio::spawn(async move {
            let prime = is_prime(i).await; // It's async - await it!
            println!("{} is prime: {}", i, prime);
        }));
    }
    for handle in handles {
        handle.await.unwrap();
    }
}
```

This works, but it has some downsides. First, you have to remember to yield - and its up to you to decide how often. Yielding too often can hurt performance, while not yielding enough can still block the runtime. Second, it makes your code more complex and harder to read.

More importantly, you're using async for something it's not really designed for: CPU-bound work.

## Option 2: Spawn Blocking

Tokio provides a function specifically for this use case: `tokio::task::spawn_blocking`. This function takes a closure and runs it on a dedicated thread pool for blocking operations. This way, you can perform blocking or CPU-intensive work without blocking the async runtime.

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

#[tokio::main]
async fn main() {
    let mut handles = vec![];
    for i in 1..=10 {
        handles.push(tokio::spawn(async move {
            let prime = tokio::task::spawn_blocking(move || is_prime(i)).await.unwrap();
            println!("{} is prime: {}", i, prime);
        }));
    }
    for handle in handles {
        handle.await.unwrap();
    }
}
```

`spawn_blocking` is actually submitting the work to a system thread pool! However, it's an easy way to combine the two paradigms.