# The Cost of a Microservice

Network calls aren't free. Even if you are pooling connections, avoiding the TCP handshake and DNS lookup, you still have the network latency and the serialization/deserialization cost. You might have the fastest network fabric in the world, but it's still slower than a function call.

There are patterns to help.

## Pooling

In the following function, which is the slowest part?

```rust
async fn fetch_url(client: &reqwest::Client, url: &str) -> Result<String, reqwest::Error> {
    let res = client.get(url).send().await?;
    let body = res.text().await?;
    Ok(body)
}
```

The `client.get(url).send().await?` line is the slowest part. The `reqwest::Client` is designed to be reused, and it pools connections. Creating a new `reqwest::Client` for each request would be *much* slower, as it would have to create a new connection each time. Instead, if you share a `reqwest::Client` instance, you can often shave hundreds of milliseconds off each request.

## Batching

Do you *need* to send just one call at a time? If you have more of a pipeline (data working its way through) rather than strict request/response, you can often batch requests together. For example, if you are sending data to a logging service, you can often send several log entries in one request. This reduces the number of network calls, and can significantly improve performance.

This helps because you are effectively only spending the network latency once for several requests, rather than once per request.

## Caching

> There are two hard things in computer science: cache invalidation and naming things.
> -- Phil Karlton

If you have data that doesn't change often, consider caching it. This can be as simple as storing the data in memory, or using a more complex caching solution like Redis or Memcached. Caching can significantly improve performance, especially for data that is expensive to compute or fetch.

You can easily use an in-process cache with Rust. The `lru` crate is a simple, fast, and thread-safe LRU (Least Recently Used) cache implementation. It's pretty trivial to bake your own.

## Does it need to be a microservice?

Sometimes, the best answer is to not make it a microservice. If you have a service that is called very frequently, and it is fast enough in-process, consider making it a library that is linked into the calling service. This eliminates the network call entirely.

Generally speaking, if a service runs faster than the network call to invoke it, consider making it a library. This is especially true for services that are called in tight loops, or that are called very frequently.