# Traits and Async

Trait and async don't always play well together. Until relatively recently, the core language didn't support async in traits at all. You can now use async functions in traits, but there are some limitations and caveats to be aware of.

Most of the time, you'll want to use `async_trait` from the `async-trait` crate. This crate provides a macro that allows you to define async functions in traits and implement them for types. It works by boxing the future returned by the async function, which has some performance implications, but it's usually worth it for the convenience. (Even with async trait support in the core language, you'll often find yourself adding Boxes!)

Here's an example of how to use `async_trait`:

```rust
use async_trait::async_trait;

#[async_trait]
trait MyAsyncTrait {
    async fn do_something(&self);
}

struct MyStruct;

#[async_trait]
impl MyAsyncTrait for MyStruct {
    async fn do_something(&self) {
        // async code here
    }
}
```