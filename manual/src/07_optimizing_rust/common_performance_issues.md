# Common Performance Issues

In this section, we'll explore some common performance issues that you might encounter in Rust programs and how to address them.

## Forgetting release mode

We mentioned it before, but it's worth remembering. Always test performance with `--release`. Debug builds are not optimized, and can be a LOT slower.

## Naive Ownership and Borrowing

When you're learning Rust, it's very common to use `clone` everywhere to make the program compile. That's ok when you're learning --- but can turn into some performance issues later.

Think in terms of what you are doing:

- Will you need the data later? If not - move it to its new home.
- If you *do* need the data later, can you borrow it instead of cloning it?
- If you can't borrow it, maybe it should be in an `Arc` (or `Rc` if you don't need thread safety).

## Excessive Println

Println! can be a performance killer. `Println!` obtains a lock on stdout each time you call it. If you have a lot of `println!` calls, this can add up to a significant amount of time. Instead, either use something like `tracing` or obtain the `stdout` lock yourself like this:

```rust
use std::io::{self, Write};
let stdout = io::stdout();
let mut handle = stdout.lock();
// Lots of printing goes here
handle.write_all(b"Hello, world!\n").unwrap();
```

## Unbuffered I/O

If you're doing a lot of I/O, make sure you're using buffered I/O. For example, use `BufReader` and `BufWriter` for file I/O instead of reading and writing directly to files.

```rust
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};

let file = File::open("foo.txt")?;
let mut reader = BufReader::new(file);
let mut contents = String::new();
reader.read_to_string(&mut contents)?;
```

## Excessive Dynamic Dispatch

If you come from an object-oriented programming background, you might be tempted to use trait objects (`Box<dyn Trait>`, `&dyn Trait`, etc.) everywhere. While they are very useful, they do come with a performance cost due to dynamic dispatch. If you find yourself using them a lot, consider if you can use generics instead. Generics are monomorphized at compile time, which can lead to better performance.