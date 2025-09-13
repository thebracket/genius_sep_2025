# Debugging Rust

Rust has a reputation for "if it compiles, it works". While this is often true, there are still times when things go wrong.

You can avoid a lot of bugs by using Rust's type system to your advantage. "New Type" (strong type) data so there's no ambiguity, use the type system to enforce invariants, and leverage Rust's ownership model to avoid data races and ensure memory safety.

With that said, things still go wrong!
