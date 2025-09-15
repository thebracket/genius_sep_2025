# Cloning

Cloning is another common performance pitfall in Rust. Cloning is explicit in Rust, so you have to call `.clone()` to make a copy of a value. This is different from languages like Python or JavaScript, where assignment creates references to the same object by default.

Cloning can be expensive, especially for large data structures. For example, consider a webserver in which you store a `String` as a state object. If you clone that `String` for each request, you will end up with a lot of unnecessary allocations and copies. Wrapping the `String` in an `Arc` (Atomic Reference Counted) pointer allows you to share the same `String` across multiple requests without cloning it.

A lot of cloning gets added when you're learning Rust, because it's the easiest way to make the program compile. As you get more comfortable with ownership and borrowing, you should be able to reduce the amount of cloning in your programs.