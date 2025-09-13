# Design 1: Shared State

Don't you wish you could write programs that don't need to keep any shared state? That's the dream, but it's not always possible---reality has a way of requiring state.

## Shared State with Mutex

You *can* create a public static Mutex, and let everyone access it:

```rust
use std::sync::Mutex;
use std::sync::LazyLock;

static DATA: LazyLock<Mutex<Vec<u8>>> = LazyLock::new(|| Mutex::new(Vec::new()));

fn main() {
    // Anything that needs it
    {
        let mut data = DATA.lock().unwrap();
        data.push(42);
    } // End scope to ensure that the lock is released
}
```

This is simple, but it has some real problems:

* ANYONE can lock the mutex at any time.
* You aren't protecting yourself against someone locking twice, deadlocking the whole system.
* You aren't protecting yourself against someone holding the lock for a long time, blocking everyone else.
* Etc! It's a big list of things that can go wrong.

As a minimum, you should create a module and make the mutex private, and provide functions to access it:

```rust
mod data {
    use std::sync::Mutex;
    use std::sync::LazyLock;

    static DATA: LazyLock<Mutex<Vec<u8>>> = LazyLock::new(|| Mutex::new(Vec::new()));

    pub fn push_data(value: u8) {
        let mut data = DATA.lock().unwrap();
        data.push(value);
    }
}

fn main() {
    // Anything that needs it
    data::push_data(42);
}
```

Now you aren't exposing your mutex to the whole world. It's still mutable global state, but at least you're preventing the worst footguns!
