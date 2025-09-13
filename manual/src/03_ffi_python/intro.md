# FFI: Python

Python has one of the easiest Foreign Function Interface (FFI) stories. Libraries such as PyO3 make it easy to write Rust code that can be called from Python. An increasing number of Python libraries make use of this. This allows developers to leverage Rust's performance and safety features while still providing a Pythonic interface.

It's a great half-way house: you write performance critical code in Rust, and you call it from Python. This is especially useful for data science and machine learning applications, where performance is often a bottleneck.

> The `Polars` library is a great example of this. It provides a DataFrame library for Python that is built on top of Rust. It is designed to be fast and efficient, and it makes use of Rust's performance features to achieve this.