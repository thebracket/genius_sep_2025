# Fearful Concurrency with Rust

Let's try and write the same program in Rust.

```rust
use std::thread;

fn main() {
    let mut counter = 0;
    let mut handles = vec![];
    for _ in 0..10 {
        let handle = thread::spawn(|| {
            for _ in 0..500_000 {
                counter += 1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("COUNTER: {}", counter);
}
```

You get a pretty enormous error message --- the key is *it won't compile*. Rust's ownership system is preventing you from shooting yourself in the foot.

Let's jump through a bunch of hoops and make it compile. Just in case Rust felt left out.

```rust
use std::thread;

static mut COUNTER: i32 = 0;

fn main() {
    let mut handles = vec![];
    for _ in 0..10 {
        let handle = thread::spawn(|| {
            for _ in 0..500_000 {
                unsafe {
                    COUNTER += 1;
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
```

If you sprinkle enough `unsafe` in there, you *can* make it fail. But if you try and remove any of those `unsafe` declarations, it won't compile. Rust is trying to protect you from yourself.