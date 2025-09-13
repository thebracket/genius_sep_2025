# Awaiting, spawning, joining and selecting

You've seen `await`---it adds the target to the task list, and yields control. There are many other ways to manage tasks. We'll go over the most common ones here. (We're using Tokio; the `futures` crate provides equivalents for most of these.)

## Spawning

You can spawn a new task using `tokio::spawn`. This takes a future and runs it in the background. It returns a `JoinHandle`, which is itself a future that you can `await` to get the result of the spawned task. Spawn starts the task immediately, and if you never `await` the `JoinHandle`, the task will run to completion in the background.

```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        sleep(Duration::from_secs(1)).await;
        println!("Task complete");
        42
    });

    println!("Keep doing other work while the task runs...");

    let result = handle.await.unwrap();
    println!("Spawned task returned: {}", result);
}
```

Spawn is often used to start long-running tasks such as actors, servers or background workers.

## Joining

Joining spawns multiple tasks and waits for all of them to complete. You can use `tokio::join!` to do this. It takes multiple futures and returns a tuple of their results.

```rust
use tokio::time::{sleep, Duration};
use tokio::task;

async fn task(num: u32) -> u32 {
    sleep(Duration::from_secs(num as u64)).await;
    num * 2
}

#[tokio::main]
async fn main() {
    let (a, b) = tokio::join!(
        task(2),
        task(3),
    );
    println!("Results: a = {}, b = {}", a, b);
}
```

## Selecting

Selecting waits for the first of multiple tasks to complete. You can use `tokio::select!` to do this. It takes multiple futures and runs them concurrently, returning the result of the first one to complete.

```rust
use tokio::time::{sleep, Duration};
use tokio::task;
use tokio::select;

async fn task(num: u32) -> u32 {
    sleep(Duration::from_secs(num as u64)).await;
    num * 2
}

#[tokio::main]
async fn main() {
    select! {
        result = task(2) => println!("Task 2 completed first with result: {}", result),
        result = task(3) => println!("Task 3 completed first with result: {}", result),
    }
}
```

This is most useful when you are waiting on multiple sources of input, such as multiple channels or network connections.

> It's extremely handy if you need to wait for "quit" messages on a broadcast channel!