# Faster Hashing

In Rust, the default hashing algorithm used by the standard library's `HashMap` and `HashSet` is SipHash. SipHash is a cryptographic hash function that provides good security properties, but it may not be the fastest option for all use cases. If you need a faster hashing algorithm for performance-critical applications, you can consider using alternative hashers like `FxHash` or `AHash`.

The `fxhash` system is used inside the Rust compiler itself. It's *not* cryptographically secure, but it's very fast and has good distribution properties for many common use cases.

To use `FxHash`, you can add the `fxhash` crate to your `Cargo.toml`:

```toml
[dependencies]
fxhash = "0.2"
```

Then, you can create a `HashMap` or `HashSet` using `FxHasher` like this:

```rust
use fxhash::FxHasher;
use std::collections::HashMap;

fn main() {
    let mut map = FxHashMap::default();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
```