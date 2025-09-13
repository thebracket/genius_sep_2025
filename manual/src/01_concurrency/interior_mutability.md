# Interior Mutability

You've probably heard that `mut` stands for "mutable". It *sort of* does, but it really means "compile time mutex".

Look closely:

```rust
use std::sync::Mutex;
use std::thread;

fn main() {
    let my_data = Mutex::new(0);
    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                for _ in 0..1000 {
                    let mut data = my_data.lock().unwrap();
                    *data += 1;
                }
            });
        }
    });
    println!("Final value: {}", *my_data.lock().unwrap());
}
```

------

Did you spot it?

------

`let my_data = Mutex::new(0);`. There's no **mut** here at all. Despite that, we're able to mutate the value inside the mutex. The mutex is providing interior mutability.

Here's another example:

```rust
use std::sync::Mutex;

struct MyStruct {
    data1: Mutex<i32>,
    data2: Mutex<i32>,
}

fn main() {
    let my_struct = MyStruct {
        data1: Mutex::new(0),
        data2: Mutex::new(0),
    };

    {
        let mut d1 = my_struct.data1.lock().unwrap();
        *d1 += 10;
    }

    {
        let mut d2 = my_struct.data2.lock().unwrap();
        *d2 += 20;
    }

    println!("data1: {}, data2: {}", *my_struct.data1.lock().unwrap(), *my_struct.data2.lock().unwrap());
}
```

There's no **mut** on `my_struct`, yet we can mutate `data1` and `data2`. That's because the `mut` keyword *doesn't*  really mean mutable. What's actually happening is that Rust is implenting the `Sync` trait.

Rust automatically implements `Sync` for types that are safe to share between threads. If everything in a type controls its own mutability (like `Mutex` does), then Rust will mark your type as `Sync`.

Because `Mutex` provides `Sync` - it *protects* its contents. It promises that it will enforce concurrency safety. So, you can share a `Mutex` between threads, even if the `Mutex` itself is not marked as `mut`.

> It's also handy if you need to have multiple threads working on parts of the same value!