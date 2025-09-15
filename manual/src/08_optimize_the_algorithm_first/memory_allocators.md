# Memory Allocators

In Rust, the default memory allocator is the system allocator, which is typically `malloc` and `free` on Unix-like systems and `HeapAlloc` and `HeapFree` on Windows. However, Rust allows you to use custom memory allocators to optimize performance for specific use cases.

Using a custom allocator can be beneficial in scenarios where you have specific memory usage patterns, such as frequent allocations and deallocations of small objects, or when you want to reduce fragmentation.

I personally use two allocators regularly on high-performance applications:
- **Jemalloc**: Jemalloc is a general-purpose memory allocator that is designed to be scalable and efficient in multi-threaded environments. It reduces fragmentation and improves cache locality, making it a popular choice for high-performance applications. JemAlloc also has good support for profiling and debugging memory usage.
- **mimalloc**: Mimalloc is a general-purpose memory allocator that is designed to be fast and efficient. It is particularly well-suited for applications that require high performance and low latency.